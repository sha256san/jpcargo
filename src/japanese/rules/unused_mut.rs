use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct UnusedMut;

impl DiagnosticRule for UnusedMut {
    fn code(&self) -> &'static str {
        "unused_mut"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Mutability
    }

    fn title(&self) -> &'static str {
        "変数に `mut` が付いていますが、値が一度も変更されていません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"variable does not need to be mutable|unused `mut`").unwrap();
        let _ = var_re;

        let summary = "変数に `mut` キーワードが指定されていますが、コード内で一度も再代入や可変操作が行われていません。";
        let reason = "Rust では最小権限の原則（不要な可変性を与えない）に従い、変更されない変数には `mut` を付けないことが推奨されます。";
        let solution = "`let mut x = ...` から `mut` を削除して、不変変数 `let x = ...` に変更してください。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("「書き換えていないので `mut` は不要です」。`let mut` を `let` に直しましょう。".to_string());
        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "warning",
            self.title(),
            "値が変更されない変数に `mut` が付与されている場合に発生します。",
            "不要な可変性を排除して安全性を高めるための警告です。",
            "`mut` を削除して不変変数にしてください。",
        )
    }
}
