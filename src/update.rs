use colored::*;
use std::process::Command;

pub struct Updater;

impl Updater {
    pub fn run_self_update() -> Result<(), Box<dyn std::error::Error>> {
        let current_version = env!("CARGO_PKG_VERSION");
        let bar = "━".repeat(68);

        println!();
        println!("{}", bar.bright_cyan());
        println!("{}", " 🔄 jpcargo セルフアップデート".bold().bright_white());
        println!("{}", bar.bright_cyan());
        println!();
        println!("  現在のバージョン: {}", current_version.bold().bright_yellow());
        println!("  ▶ 最新リリースの確認および更新を実行中...");
        println!();

        // curl で最新の install.sh を取得して bash で実行
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh?v=$(date +%s) | bash")
            .status()?;

        if status.success() {
            println!();
            println!("{}", "✅ jpcargo は最新バージョンに正常にアップデートされました！".bold().bright_green());
            println!("{}", bar.bright_cyan());
            println!();
        } else {
            println!();
            println!("{}", "❌ アップデート中にエラーが発生しました。".bold().bright_red());
            println!("手動で以下のコマンドを実行してください:");
            println!("  curl -fsSL https://raw.githubusercontent.com/sha256san/jpcargo/main/install.sh | bash");
            println!("{}", bar.bright_cyan());
            println!();
        }

        Ok(())
    }
}
