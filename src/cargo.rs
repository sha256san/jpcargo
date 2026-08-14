use colored::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use crate::diagnostic::{parse_line, CargoMessage};
use crate::japanese::{JapaneseLevel, Translator};
use crate::renderer::TerminalRenderer;

pub struct CargoRunner {
    #[allow(dead_code)]
    pub translator: Translator,
    pub renderer: TerminalRenderer,
    pub level: JapaneseLevel,
}

impl CargoRunner {
    pub fn new(show_original: bool, quiet: bool, verbose: bool, level: JapaneseLevel) -> Self {
        Self {
            translator: Translator::new(),
            renderer: TerminalRenderer::new(show_original, quiet, verbose, level),
            level,
        }
    }

    pub fn execute(&self, subcommand: &str, args: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("cargo");
        cmd.arg(subcommand);

        // --message-format=json を追加
        let mut has_message_format = false;
        let mut passed_args = Vec::new();

        for arg in args {
            if arg.starts_with("--message-format") {
                has_message_format = true;
            }
            passed_args.push(arg.clone());
        }

        if !has_message_format && subcommand != "doc" && subcommand != "fix" {
            cmd.arg("--message-format=json");
        }

        for arg in &passed_args {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            format!("Cargoの起動に失敗しました（cargoがインストールされているか確認してください）: {}", e)
        })?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut error_count = 0;
        let mut warning_count = 0;

        // stdout のストリーム処理
        let stdout_handle = std::thread::spawn({
            let translator = Translator::new();
            let renderer = TerminalRenderer::new(
                self.renderer.show_original,
                self.renderer.quiet,
                self.renderer.verbose,
                self.level,
            );

            move || {
                let reader = BufReader::new(stdout);
                let mut errs = 0;
                let mut warns = 0;
                let mut collected_diags = Vec::new();

                for line_res in reader.lines() {
                    if let Ok(line) = line_res {
                        if let Some(msg) = parse_line(&line) {
                            if let CargoMessage::CompilerMessage { message, .. } = msg {
                                if message.level == "error" {
                                    errs += 1;
                                    let jd = translator.translate(&message);
                                    renderer.render(&jd);
                                    collected_diags.push(jd);
                                } else if message.level == "warning" && !renderer.quiet {
                                    warns += 1;
                                    let jd = translator.translate(&message);
                                    renderer.render(&jd);
                                    collected_diags.push(jd);
                                }
                            }
                        } else {
                            // JSON でない行（実行バイナリの通常出力など）はそのまま表示
                            println!("{}", line);
                        }
                    }
                }

                // すべてのエラー・警告の表示後にサマリー一覧テーブルを表示
                if !collected_diags.is_empty() {
                    renderer.render_summary_table(&collected_diags);
                }

                (errs, warns)
            }
        });

        // stderr の処理
        let stderr_handle = std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line_res in reader.lines() {
                if let Ok(line) = line_res {
                    let trimmed = line.trim();

                    // リンカーエラー検知
                    if trimmed.contains("linker `cc` not found") || trimmed.contains("linker `gcc` not found") || trimmed.contains("linker `clang` not found") {
                        let bar = "━".repeat(68);
                        println!();
                        println!("{}", bar.bright_red());
                        println!("{} {}", "環境エラー [LINKER]".white().on_red().bold(), "Cコンパイラ/リンカーが見つかりません".bold());
                        println!("{}", bar.bright_red());
                        println!("\n【原因】");
                        println!("  Rustコード自体の問題ではなく、OS環境にリンカー（Cコンパイラ）がインストールされていません。");
                        println!("\n【解決方法】");
                        println!("  以下のコマンドを実行してビルド必須ツールをインストールしてください:");
                        println!("    Debian/Ubuntu: {}", "sudo apt update && sudo apt install build-essential".bright_green());
                        println!("    macOS:         {}", "xcode-select --install".bright_green());
                        println!("    Fedora/RHEL:   {}", "sudo dnf groupinstall 'Development Tools'".bright_green());
                        println!();
                        println!("{}", bar.bright_red());
                        println!();
                        continue;
                    }

                    // Cargo依存解決エラー検知
                    if trimmed.starts_with("error: failed to select a version for the requirement") || trimmed.starts_with("error: no matching package named") {
                        let bar = "━".repeat(68);
                        println!();
                        println!("{}", bar.bright_red());
                        println!("{} {}", "Cargoエラー [DEPENDENCY]".white().on_red().bold(), "依存パッケージの解決に失敗しました".bold());
                        println!("{}", bar.bright_red());
                        println!("\n【概要】");
                        println!("  {}", trimmed);
                        println!("\n【確認事項】");
                        println!("  1. Cargo.toml に指定したパッケージ名やバージョン番号が正しいか確認してください。");
                        println!("  2. インターネット接続を確認してください。");
                        println!("  3. `cargo update` を試してください。");
                        println!();
                        println!("{}", bar.bright_red());
                        println!();
                        continue;
                    }

                    // コンパイラが直接吐く英語エラー/警告ヘッダー行は日本語レンダラーが出すのでスキップ
                    if trimmed.starts_with("error[E") || trimmed.starts_with("warning:") || trimmed.starts_with("For more information about this error") {
                        continue;
                    }
                    if !trimmed.is_empty() {
                        eprintln!("{}", line);
                    }
                }
            }
        });

        if let Ok((errs, warns)) = stdout_handle.join() {
            error_count += errs;
            warning_count += warns;
        }
        let _ = stderr_handle.join();

        let status = child.wait()?;
        let exit_code = status.code().unwrap_or(1);

        if self.renderer.verbose && (error_count > 0 || warning_count > 0) {
            println!(
                "jpcargo: 検出されたエラー: {} 件, 警告: {} 件",
                error_count, warning_count
            );
        }

        Ok(exit_code)
    }
}
