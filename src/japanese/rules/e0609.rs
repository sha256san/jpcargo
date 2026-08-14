use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0609;

impl DiagnosticRule for E0609 {
    fn code(&self) -> &'static str {
        "E0609"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "構造体に存在しないフィールドにアクセスしようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let field_re = Regex::new(r"no field `(?P<field>[^`]+)` on type `(?P<type>[^`]+)`").unwrap();
        let (field_name, type_name) = if let Some(caps) = field_re.captures(&diag.message) {
            (
                caps.name("field").map(|m| m.as_str()).unwrap_or("フィールド"),
                caps.name("type").map(|m| m.as_str()).unwrap_or("型"),
            )
        } else {
            ("指定のフィールド", "指定の型")
        };

        let summary = format!(
            "型「{}」にはフィールド「{}」は定義されていません。",
            type_name, field_name
        );

        let reason = "フィールド名のタイポ（綴り間違い）、または想定している型が異なっている可能性があります。";
        let solution = format!("構造体の定義を確認し、正しいフィールド名を使用してください。");

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
            "構造体に定義されていないフィールド名を `.field` で参照しようとした場合に発生します。",
            "タイポや構造体定義の変更漏れが原因です。",
            "構造体の定義とフィールド名を確認してください。",
        )
    }
}
