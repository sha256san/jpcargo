use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0596;

impl DiagnosticRule for E0596 {
    fn code(&self) -> &'static str {
        "E0596"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Mutability
    }

    fn title(&self) -> &'static str {
        "変数がミュータブル（可変）として宣言されていません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot assign to `(?P<var>[^`]+)`").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の変数");

        let summary = format!(
            "変数「{}」がミュータブル（可変）として宣言されていないため、値を変更・再代入できません。",
            var_name
        );

        let reason = "Rust では、`let` で宣言された変数はデフォルトでイミュータブル（不変）です。\n\
            イミュータブルな変数に一度値を代入すると、後から値を変更することはできません。\n\
            これにより、意図しないデータの書き換えやバグを防ぎます。";

        let solution = format!(
            "変数宣言に `mut` キーワードを追加して、明示的にミュータブル（変更可能）にしてください。\n\
            例: `let mut {} = ...;`",
            var_name
        );

        let before = format!("let {} = ...;", var_name);
        let after = format!("let mut {} = ...;", var_name);

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.example_diff = Some((before, after));
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            if child.level == "help" {
                jd.suggestions.push(format!("ヒント: {}", child.message));
            }
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "不変（immutable）として宣言された変数に値を代入・変更しようとすると発生します。",
            "Rust の変数はデフォルトで不変です。安全性のために変更が禁止されています。",
            "変数定義時に `let mut` を使用して可変であることを宣言してください。",
        )
    }
}
