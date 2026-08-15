use colored::*;
use crate::japanese::template::JapaneseDiagnostic;

pub struct TerminalRenderer {
    pub show_original: bool,
    pub quiet: bool,
    pub verbose: bool,
}

impl TerminalRenderer {
    pub fn new(show_original: bool, quiet: bool, verbose: bool) -> Self {
        Self {
            show_original,
            quiet,
            verbose,
        }
    }

    pub fn render(&self, jd: &JapaneseDiagnostic) {
        let is_warning = jd.level == "warning";
        let header_color = if is_warning {
            "yellow"
        } else {
            "bright red"
        };

        let badge = if is_warning {
            "警告".black().on_yellow().bold()
        } else {
            "エラー".white().on_red().bold()
        };

        let bar = "━".repeat(68);

        println!();
        println!("{}", bar.color(header_color));
        println!(
            "{} [{}] {} ({})",
            badge,
            jd.code.bold().bright_cyan(),
            jd.title.bold(),
            jd.category.name_ja().dimmed()
        );
        println!("{}", bar.color(header_color));
        println!();

        // 発生箇所
        if let Some(loc) = &jd.location {
            println!("{}", "【発生箇所】".bold().bright_yellow());
            println!("  {} {}", "-->".bright_blue().bold(), loc.underline());
            println!();
        }

        // コードスニペット
        if !self.quiet {
            if let Some(snippet) = &jd.snippet {
                println!("{}", "【コード】".bold().bright_yellow());
                for line in snippet.lines() {
                    if line.contains('^') {
                        println!("  {}", line.bright_red().bold());
                    } else if line.contains('|') {
                        let parts: Vec<&str> = line.splitn(2, '|').collect();
                        if parts.len() == 2 {
                            println!("  {}|{}", parts[0].bright_blue(), parts[1]);
                        } else {
                            println!("  {}", line);
                        }
                    } else {
                        println!("  {}", line);
                    }
                }
                println!();
            }
        }

        // quietモードでなければ修正例を表示
        if !self.quiet {
            let has_options = !jd.fix_options.is_empty();
            let has_diff = jd.example_diff.is_some();
            let has_solution = !jd.solution.is_empty();

            if has_options || has_diff || has_solution {
                println!("{}", "【修正例】".bold().bright_yellow());

                if has_options {
                    for (i, opt) in jd.fix_options.iter().enumerate() {
                        if !opt.description.is_empty() {
                            println!("  // {}", opt.description.cyan());
                        }
                        if let Some((before, after)) = &opt.diff {
                            println!("  {} {}", "-".red().bold(), before.red());
                            println!("  {} {}", "+".green().bold(), after.green());
                        } else if let Some(code) = &opt.code {
                            for line in code.lines() {
                                println!("  {}", line.bright_green());
                            }
                        }
                        if i + 1 < jd.fix_options.len() {
                            println!();
                        }
                    }
                } else {
                    // 修正例 Diff
                    if let Some((before, after)) = &jd.example_diff {
                        println!("  {} {}", "-".red().bold(), before.red());
                        println!("  {} {}", "+".green().bold(), after.green());
                    }

                    // 一言解説
                    if has_solution {
                        for line in jd.solution.lines() {
                            println!("  {}", line.bright_green());
                        }
                    }
                }
                println!();
            }
        }

        // エラーコード詳細コマンドの案内
        if jd.code.starts_with('E') && jd.code.len() == 5 {
            println!(
                "  エラーコード詳細: {}",
                format!("jpcargo explain {}", jd.code).bright_cyan().bold()
            );
            println!();
        }

        // 原文表示（--original または 未対応エラー時）
        if self.show_original {
            if let Some(orig) = &jd.original_message {
                println!("{}", "【コンパイラ原文 (English)】".bold().dimmed());
                println!("  {}", orig.dimmed());
                println!();
            }
        }

        println!("{}", bar.color(header_color));
        println!();
    }

    pub fn render_summary_table(&self, diagnostics: &[JapaneseDiagnostic]) {
        if diagnostics.is_empty() {
            return;
        }

        let total_errors = diagnostics.iter().filter(|d| d.level == "error").count();
        let total_warnings = diagnostics.iter().filter(|d| d.level == "warning").count();

        let bar = "━".repeat(78);
        println!();
        println!("{}", bar.bright_cyan());
        println!(
            "{} (エラー: {} 件, 警告: {} 件)",
            " 診断サマリー一覧".bold().bright_white(),
            total_errors.to_string().bright_red().bold(),
            total_warnings.to_string().yellow().bold()
        );
        println!("{}", bar.bright_cyan());

        for (idx, jd) in diagnostics.iter().enumerate() {
            let num = idx + 1;
            let is_warning = jd.level == "warning";
            let type_str = if is_warning {
                "警告".yellow().bold()
            } else {
                "エラー".bright_red().bold()
            };

            let loc = jd.location.as_deref().unwrap_or("-");
            let code = jd.code.as_str().bright_cyan().bold();
            let title = &jd.title;

            // ユーザー指定のフォーマット: 1|src/main.rs:19:5|エラー|E0502|タイトル
            println!(
                " {:2} | {:<22} | {:<4} | {:<18} | {}",
                num,
                loc.underline(),
                type_str,
                code,
                title
            );
        }

        println!("{}", bar.bright_cyan());
        println!();
    }
}
