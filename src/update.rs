use colored::*;
use std::process::Command;

pub struct Updater;

impl Updater {
    pub fn run_self_update() -> Result<(), Box<dyn std::error::Error>> {
        let current_version = env!("CARGO_PKG_VERSION");
        let bar = "━".repeat(68);

        println!();
        println!("{}", bar.bright_cyan());
        println!("{}", " jpcargo アップデート".bold().bright_white());
        println!("{}", bar.bright_cyan());
        println!();
        println!("  現在バージョン: {}", current_version.bold().bright_yellow());
        println!("  最新版を取得中...");
        println!();

        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh?v=$(date +%s) | bash")
            .status()?;

        if status.success() {
            println!();
            println!("{}", "  [v] 最新バージョンへの更新が完了しました".bold().bright_green());
            println!("{}", bar.bright_cyan());
            println!();
        } else {
            println!();
            println!("{}", "  [x] アップデートに失敗しました".bold().bright_red());
            println!("  手動実行コマンド:");
            println!("    curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh | bash");
            println!("{}", bar.bright_cyan());
            println!();
        }

        Ok(())
    }
}
