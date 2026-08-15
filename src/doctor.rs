use colored::*;
use std::process::Command;

pub struct Doctor;

impl Doctor {
    pub fn run_diagnosis() {
        let bar = "━".repeat(68);
        println!();
        println!("{}", bar.bright_cyan());
        println!("{}", " jpcargo doctor — Rust & Cargo 開発環境診断".bold().bright_white());
        println!("{}", bar.bright_cyan());
        println!();

        // 1. Rust コンパイラ (rustc)
        Self::check_command("rustc", &["--version"], "Rust コンパイラ (rustc)", true);

        // 2. パッケージマネージャ (cargo)
        Self::check_command("cargo", &["--version"], "Cargo パッケージマネージャ", true);

        // 3. ツールチェーン管理 (rustup)
        Self::check_command("rustup", &["--version"], "rustup ツールチェーンマネージャ", false);

        // 4. リンカー・Cコンパイラ (cc / gcc / clang)
        let cc_ok = Self::check_linker();

        // 5. システム環境
        println!("{}", "【システム環境情報】".bold().bright_yellow());
        println!("  - OS:            {}", std::env::consts::OS.bold());
        println!("  - アーキテクチャ: {}", std::env::consts::ARCH.bold());
        println!("  - ファミリ:      {}", std::env::consts::FAMILY.bold());
        println!();

        // 総合判定
        println!("{}", "【診断結果】".bold().bright_yellow());
        if cc_ok {
            println!("  {} すべての必須コンポーネントが正常に利用可能です。", "[v] [正常]".bright_green().bold());
        } else {
            println!("  {} Cコンパイラ/リンカーが見つかりません。ビルド時にリンクエラーが発生する可能性があります。", "[!] [要確認]".bright_yellow().bold());
            println!("     Debian/Ubuntu: `sudo apt install build-essential`");
            println!("     macOS:         `xcode-select --install`");
            println!("     Fedora/RHEL:   `sudo dnf groupinstall 'Development Tools'`");
        }

        println!();
        println!("{}", bar.bright_cyan());
        println!();
    }

    fn check_command(cmd: &str, args: &[&str], label: &str, required: bool) -> bool {
        print!("  Checking {:<30} ... ", label);
        match Command::new(cmd).args(args).output() {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let first_line = stdout.lines().next().unwrap_or("").trim();
                println!("{} ({})", "OK".bright_green().bold(), first_line.dimmed());
                true
            }
            _ => {
                if required {
                    println!("{}", "[x] 見つかりません (必須)".bright_red().bold());
                } else {
                    println!("{}", "[-] 未インストール (任意)".yellow());
                }
                false
            }
        }
    }

    fn check_linker() -> bool {
        print!("  Checking {:<30} ... ", "C リンカー (cc / gcc / clang)");
        let linkers = ["cc", "gcc", "clang"];
        for l in &linkers {
            if let Ok(output) = Command::new(l).arg("--version").output() {
                if output.status.success() {
                    let first_line = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    println!("{} ({} - {})", "OK".bright_green().bold(), l.bold(), first_line.dimmed());
                    return true;
                }
            }
        }
        println!("{}", "[x] 見つかりません".bright_red().bold());
        false
    }
}
