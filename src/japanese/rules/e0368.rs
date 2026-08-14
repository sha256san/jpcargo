use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0368;

impl DiagnosticRule for E0368 {
    fn code(&self) -> &'static str {
        "E0368"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Operator
    }

    fn title(&self) -> &'static str {
        "代入演算子（+=, -=, *= 等）をサポートしていない型に適用しようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "該当の型に対して代入演算子（`+=`, `-=` など）が実装されていないか、変数が `mut` で宣言されていません。";
        let reason = "`+=` などの演算子には `std::ops::AddAssign` などのトレイトの実装と、左辺が可変変数であることが必要です。";
        let solution = "1. 変数が `let mut` で可変として宣言されているか確認してください。\n2. 型に対応する代入演算子トレイトが実装されているか確認してください。";

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
            "複合代入演算子（`+=` 等）がサポートされていない型に対して使用された場合に発生します。",
            "`AddAssign` トレイトの未実装や、変数のイミュータブル性が原因です。",
            "可変性の確認または演算子トレイトの実装を行ってください。",
        )
    }
}
