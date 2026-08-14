use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0026;

impl DiagnosticRule for E0026 {
    fn code(&self) -> &'static str {
        "E0026"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "構造体パターンで存在しないフィールド名が指定されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "構造体のパターンマッチング時に、構造体定義に存在しないフィールド名が書かれています。";
        let reason = "フィールド名のタイポ、または構造体定義との不一致が原因です。";
        let solution = "構造体の定義を確認し、正しいフィールド名を使用してください。";

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

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "パターン内で存在しない構造体フィールドを指定した場合に発生します。",
            "タイポや構造体定義との相違が原因です。",
            "正しいフィールド名に修正してください。",
        )
    }
}
