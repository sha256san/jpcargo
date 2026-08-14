use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0600;

impl DiagnosticRule for E0600 {
    fn code(&self) -> &'static str {
        "E0600"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Operator
    }

    fn title(&self) -> &'static str {
        "単項演算子（- や ! 等）をサポートしていない型に適用しようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "単項マイナス（`-x`）や論理否定（`!x`）がサポートされていない型に対して適用されました（例: 符号なし整数 `u32` への `-` など）。";
        let reason = "符号なし整数（`u8`, `u32` 等）は負の数を表現できないため、単項マイナス `-` は定義されていません。";
        let solution = "符号付き整数（`i32`, `i64` 等）に型変換してから符号反転を行うか、演算子の対象を確認してください。";

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
            "単項演算子（`-` や `!`）が定義されていない型に対して使用された場合に発生します。",
            "符号なし整数への単項マイナス適用などが主な原因です。",
            "符号付き整数（`i32` 等）を使用してください。",
        )
    }
}
