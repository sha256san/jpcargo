use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0106;

impl DiagnosticRule for E0106 {
    fn code(&self) -> &'static str {
        "E0106"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "関数の戻り値などにライフタイム指定子（'a）が不足しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "参照を戻り値として返す関数において、コンパイラがライフタイム（生存期間）を自動推論（ライフタイム省略規則）できませんでした。";
        let reason = "関数の引数に複数の参照がある場合（例: `fn foo(x: &str, y: &str) -> &str`）、戻り値の参照が `x` と `y` のどちらの寿命に紐づくのかをコンパイラが判断できないため、明示的なジェネリックライフタイムパラメータ（`'a`）が必要です。";
        let solution = "関数定義にライフタイムパラメータを追加してください。\n例: `fn foo<'a>(x: &'a str, y: &'a str) -> &'a str`";

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
            "参照を返す関数の戻り値に対して、ライフタイムが推論不能な場合に発生します。",
            "入力参照と出力参照の寿命関係をコンパイラに明示する必要があります。",
            "`<'a>` ライフタイム指定子を付与してください。",
        )
    }
}
