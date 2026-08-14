use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0271;

impl DiagnosticRule for E0271 {
    fn code(&self) -> &'static str {
        "E0271"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "Trait の関連型（Associated Type）が要求されている型と一致しません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "Trait に定義された関連型（例: `Iterator::Item` 等）が、関数や構造体が期待している具体的な型と一致していません。";
        let reason = "ジェネリック関数などが `T: Iterator<Item = i32>` を要求しているのに対し、`Item = String` であるイテレータを渡した場合などに発生します。";
        let solution = "1. 要求されている関連型に変換するか（例: `.map(...)` など）、\n2. 関数の型境界（トレイト境界）を修正してください。";

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
            "Traitの関連型が、要求されている型制約と一致しない場合に発生します。",
            "イテレータの要素型などの不一致が主な原因です。",
            "関連型の制約に合致するようにデータ型を変換してください。",
        )
    }
}
