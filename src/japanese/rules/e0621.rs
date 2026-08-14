use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0621;

impl DiagnosticRule for E0621 {
    fn code(&self) -> &'static str {
        "E0621"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "関数の引数と戻り値のライフタイム関係が一致していません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "戻り値として返そうとしている引数の参照に、戻り値と同じライフタイム注釈（`'a`）が付いていません。";
        let reason = "Rust では、戻り値が特定のライフタイム `'a` を持つ場合、その元となる引数も同一の `'a` ライフタイムを持っている必要があります。";
        let solution = "引数の参照にも戻り値と同じライフタイム注釈を付与してください。\n例: `fn foo<'a>(x: &'a str, y: &'a str) -> &'a str`";

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
            "戻り値に要求されるライフタイムが、渡された引数のライフタイムと一致しない場合に発生します。",
            "ライフタイムパラメータの指定漏れが原因です。",
            "該当の引数に適切なライフタイム（`'a` 等）を指定してください。",
        )
    }
}
