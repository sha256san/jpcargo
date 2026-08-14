use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0608;

impl DiagnosticRule for E0608 {
    fn code(&self) -> &'static str {
        "E0608"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "配列やスライス以外の型に対してインデックスアクセス（`[index]`）を行おうとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let type_re = Regex::new(r"cannot index into a value of type `(?P<type>[^`]+)`").unwrap();
        let type_name = type_re
            .captures(&diag.message)
            .and_then(|c| c.name("type"))
            .map(|m| m.as_str())
            .unwrap_or("該当の型");

        let summary = format!(
            "型「{}」は `Index` トレイトを実装していないため、`[0]` のような角括弧によるインデックス参照はできません。",
            type_name
        );

        let reason = "`[index]` 記法は配列、ベクタ（`Vec<T>`）、スライス（`&[T]`）、または `std::ops::Index` を実装した型でのみ利用可能です。";
        let solution = "タプルの場合は `.0`, `.1` を使用し、構造体の場合はフィールド名（`.field`）を使用してください。";

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
            "インデックス参照がサポートされていない型に対して `[i]` を使用した場合に発生します。",
            "`Index` トレイトが未実装の型へのアクセスが原因です。",
            "タプルの場合は `.0` などのドット記法を使用してください。",
        )
    }
}
