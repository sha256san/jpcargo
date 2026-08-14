use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0069;

impl DiagnosticRule for E0069 {
    fn code(&self) -> &'static str {
        "E0069"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Function
    }

    fn title(&self) -> &'static str {
        "戻り値型が `()` の関数で、値を伴う `return <値>;` が使用されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "戻り値のない関数（暗黙的に `()` を返す関数）の中で、値を返す `return` 文が書かれています。";
        let reason = "関数の戻り値シグネチャと、実際に `return` で返そうとしている値の型が不一致です。";
        let solution = "1. 単に早期リターンしたい場合は、値なしの `return;` を使用してください。\n2. 値を返したい場合は、関数のシグネチャに戻り値型を指定してください（例: `fn foo() -> i32`）。";

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
            "戻り値型が `()` である関数から値を返そうとした場合に発生します。",
            "戻り値の型シグネチャの欠落、または `return;` の誤用が原因です。",
            "`return;` とするか、関数定義に戻り値型（`-> Type`）を追加してください。",
        )
    }
}
