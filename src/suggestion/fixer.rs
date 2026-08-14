use std::process::Command;
use colored::*;
use crate::diagnostic::Diagnostic;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub message: String,
    pub replacement: Option<String>,
}

#[allow(dead_code)]
pub fn extract_suggestions(diag: &Diagnostic) -> Vec<FixSuggestion> {
    let mut results = Vec::new();

    // primary spans からの suggestion
    for span in &diag.spans {
        if let Some(replacement) = &span.suggested_replacement {
            results.push(FixSuggestion {
                message: span.label.clone().unwrap_or_else(|| "修正候補".to_string()),
                replacement: Some(replacement.clone()),
            });
        }
    }

    // children (help/note) からの suggestion
    for child in &diag.children {
        if child.level == "help" {
            let replacement = child.spans.iter().find_map(|s| s.suggested_replacement.clone());
            results.push(FixSuggestion {
                message: child.message.clone(),
                replacement,
            });
        }
    }

    results
}

pub fn run_cargo_fix(args: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
    println!("\n{}", "🔧 jpcargo fix — rustc の自動修正候補（MachineApplicable）を適用中...".bold().bright_cyan());

    let mut cmd = Command::new("cargo");
    cmd.arg("fix");
    cmd.arg("--allow-no-vcs");

    for arg in args {
        cmd.arg(arg);
    }

    let status = cmd.status().map_err(|e| {
        format!("cargo fix の実行に失敗しました: {}", e)
    })?;

    if status.success() {
        println!("{}", "✅ 自動修正が完了しました。".bold().bright_green());
    } else {
        println!("{}", "⚠️ 一部の修正を自動適用できませんでした。手動での確認が必要です。".bold().bright_yellow());
    }

    Ok(status.code().unwrap_or(1))
}
