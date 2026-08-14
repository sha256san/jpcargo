use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0391;

impl DiagnosticRule for E0391 {
    fn code(&self) -> &'static str {
        "E0391"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "型の定義またはトレイト制約に循環依存が検出されました (Cycle detected)"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "型エイリアスやトレイト定義において、A が B に依存し、B が A に依存するような循環参照が発生しています（例: `type A = B; type B = A;`）。";
        let reason = "コンパイラが型のサイズや定義を解決する際に無限ループに陥ってしまうためです。";
        let solution = "循環している型定義の関係を解消し、一方向の依存関係になるように再設計してください。";

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
            "型やトレイトの定義が互いに循環参照している場合に発生します。",
            "無限の依存ループが原因です。",
            "型エイリアスやトレイト定義の依存関係を整理してください。",
        )
    }
}
