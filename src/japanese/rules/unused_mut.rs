use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
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
        let solution = "";

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
        jd.original_message = Some(diag.message.clone());

        // 複数の修正方法とコード例（日本語コメント付き）
        jd.add_fix_option(FixOption::diff(
            "方法1: 不要な `mut` を削除して不変変数にする",
            "let mut x = ...;",
            "let x = ...;",
        ));

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "warning",
            self.title(),
            "値が変更されない変数に `mut` が付与されている場合に発生します。",
            "不要な可変性を排除して安全性を高めるための警告です。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: 不要な `mut` を削除する",
            "let mut x = ...;",
            "let x = ...;",
        ));
        jd
    }
}
