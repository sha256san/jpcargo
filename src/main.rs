mod cargo;
mod cli;
mod diagnostic;
mod doctor;
mod japanese;
mod renderer;
mod rustc;
mod suggestion;

use clap::Parser;
use colored::*;
use cli::{Cli, Commands};
use cargo::CargoRunner;
use doctor::Doctor;
use japanese::{ExplanationService, Translator};
use renderer::TerminalRenderer;
use rustc::RustcRunner;
use suggestion::run_cargo_fix;

fn main() {
    let cli = Cli::parse();
    let runner = CargoRunner::new(cli.original, cli.quiet, cli.verbose, cli.level);
    let renderer = TerminalRenderer::new(cli.original, cli.quiet, cli.verbose, cli.level);

    match cli.command {
        Commands::Run { args } => {
            let code = runner.execute("run", &args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Build { args } => {
            let code = runner.execute("build", &args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Check { args } => {
            let code = runner.execute("check", &args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Test { args } => {
            let code = runner.execute("test", &args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Clippy { args } => {
            let code = runner.execute("clippy", &args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Doc { args } => {
            let code = runner.execute("doc", &args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Fix { args } => {
            let code = run_cargo_fix(&args).unwrap_or(1);
            std::process::exit(code);
        }
        Commands::Doctor => {
            Doctor::run_diagnosis();
        }
        Commands::Explain { code } => {
            let translator = Translator::new();
            if let Some(jd) = translator.explain_code(&code) {
                renderer.render(&jd);
            } else {
                println!(
                    "{} エラーコード「{}」の日本語解説はまだ登録されていません。",
                    "情報:".bright_yellow().bold(),
                    code.bright_cyan()
                );
                println!("利用可能なエラーコード一覧を見るには `jpcargo list` を実行してください。");
            }
        }
        Commands::Search { query } => {
            let results = ExplanationService::search(&query);
            println!(
                "\n{} 「{}」の検索結果: {} 件\n",
                "🔍".bold(),
                query.bright_yellow(),
                results.len().to_string().bold()
            );

            if results.is_empty() {
                println!("一致するエラーコードや解説は見つかりませんでした。");
            } else {
                for jd in results {
                    println!(
                        "  [{}] {} ({})",
                        jd.code.bold().bright_cyan(),
                        jd.title.bold(),
                        jd.category.name_ja().dimmed()
                    );
                    println!("      {}\n", jd.summary);
                }
            }
        }
        Commands::List => {
            let all = ExplanationService::list_all();
            println!("\n{}", "=== jpcargo 対応済みエラーコード一覧 ===".bold().bright_green());
            println!("登録エラー数: {} 件\n", all.len().to_string().bold());

            for jd in all {
                println!(
                    "  [{}] {:<28} - {}",
                    jd.code.bold().bright_cyan(),
                    jd.category.name_ja().dimmed(),
                    jd.title
                );
            }
            println!("\n詳細な解説を表示するには: `jpcargo explain <CODE>` (例: `jpcargo explain E0596`)\n");
        }
        Commands::Stats => {
            let (total, cat_map) = ExplanationService::stats();
            println!("\n{}", "=== jpcargo 診断統計 ===".bold().bright_green());
            println!("対応エラー総数: {} 件\n", total.to_string().bold().bright_cyan());
            println!("{}", "【カテゴリ別内訳】".bold());

            let mut sorted_cats: Vec<_> = cat_map.into_iter().collect();
            sorted_cats.sort_by(|a, b| b.1.cmp(&a.1));

            for (cat, count) in sorted_cats {
                println!("  - {:<32} : {} 件", cat.name_ja(), count);
            }
            println!();
        }
        Commands::Rustc { file, args } => {
            let rustc_runner = RustcRunner::new(cli.original, cli.quiet, cli.verbose, cli.level);
            let code = rustc_runner.compile_file(&file, &args).unwrap_or(1);
            std::process::exit(code);
        }
    }
}
