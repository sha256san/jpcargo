use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct NonCamelCaseTypes;

impl DiagnosticRule for NonCamelCaseTypes {
    fn code(&self) -> &'static str {
        "non_camel_case_types"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Style
    }

    fn title(&self) -> &'static str {
        "型名（構造体・enum・Trait）が Rust 標準の命名規則（`CamelCase`）に違反しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let name_re = Regex::new(r"should have an upper camel case name: `(?P<name>[^`]+)`").unwrap();
        let name_str = name_re
            .captures(&diag.message)
            .and_then(|c| c.name("name"))
            .map(|m| m.as_str())
            .unwrap_or("該当の型名");

        let summary = format!(
            "型名「{}」はアッパーキャメルケース（単語の先頭を大文字）になっていません。",
            name_str
        );

        let reason = "Rust では型（構造体、enum、Trait、型エイリアス）の名前はアッパーキャメルケース（例: `MyStruct`, `HttpServer`）にする規則があります。";
        let solution = "単語の先頭を大文字にしたキャメルケースにリネームしてください。";

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
            "warning",
            self.title(),
            "構造体やenum等の型名がキャメルケース（`CamelCase`）になっていない場合に発生します。",
            "コードの可読性と一貫性のための公式規約です。",
            "`MyType` のように単語の先頭を大文字にして命名してください。",
        )
    }
}
