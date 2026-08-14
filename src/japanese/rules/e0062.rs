use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0062;

impl DiagnosticRule for E0062 {
    fn code(&self) -> &'static str {
        "E0062"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Function
    }

    fn title(&self) -> &'static str {
        "構造体インスタンス化やパターンで同じフィールド名が複数回指定されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "構造体の初期化時に、同一のフィールド名に対して重複して値を代入しようとしています。";
        let reason = "各フィールドの値は一意に定まる必要があります。";
        let solution = "重複して記述されているフィールド指定を削除してください。";

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
            "構造体の初期化で、同一フィールドが重複して指定された場合に発生します。",
            "フィールド値の重複代入が原因です。",
            "重複したフィールド指定を取り除いてください。",
        )
    }
}
