use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0027;

impl DiagnosticRule for E0027 {
    fn code(&self) -> &'static str {
        "E0027"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "構造体パターンで未指定のフィールドが存在します"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "パターンマッチ（`match` や `let Struct { ... }`）で構造体を分解する際、すべてのフィールドが列挙されていません。";
        let reason = "すべてのフィールドをマッチさせるか、残りのフィールドを無視することを明示する必要があります。";
        let solution = "1. 不足しているフィールドをパターンに追加するか、\n2. 残りのフィールドを無視するために `..` をパターン末尾に追加してください（例: `Struct { a, .. }`）。";

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
            "構造体パターンマッチで一部のフィールドが省略されている場合に発生します。",
            "意図せぬフィールド無視を防ぐためのチェックです。",
            "パターンに `..` を追加して明示的に残りを無視してください。",
        )
    }
}
