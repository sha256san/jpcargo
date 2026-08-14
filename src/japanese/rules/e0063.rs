use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0063;

impl DiagnosticRule for E0063 {
    fn code(&self) -> &'static str {
        "E0063"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "構造体の初期化時に必須フィールドが指定されていません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let field_re = Regex::new(r"missing field[s]? `(?P<field>[^`]+)`").unwrap();
        let missing = field_re
            .captures(&diag.message)
            .and_then(|c| c.name("field"))
            .map(|m| m.as_str())
            .unwrap_or("必須フィールド");

        let summary = format!(
            "構造体のインスタンス化時に、フィールド「{}」への値の代入が不足しています。",
            missing
        );

        let reason = "Rust の構造体は、すべてのフィールドが完全に初期化される必要があります。未初期化フィールドを持つことは許されません。";

        let solution = format!(
            "1. 不足しているフィールド `{}: <値>` を追加してください。\n\
            2. `Default` トレイトを実装している場合は、構造体更新構文 `..Default::default()` を末尾に追加してください。",
            missing
        );

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
            "構造体のインスタンス生成時に、定義されたフィールドの指定が漏れている場合に発生します。",
            "全フィールドの完全初期化が必要です。",
            "不足しているフィールドを記述するか、`..Default::default()` を使用してください。",
        )
    }
}
