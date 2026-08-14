use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0571;

impl DiagnosticRule for E0571 {
    fn code(&self) -> &'static str {
        "E0571"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Function
    }

    fn title(&self) -> &'static str {
        "A break statement with an argument appeared in a non-loop loop"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let mut jd = self.general_explanation();
        jd.level = diag.level.clone();
        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "A break statement with an argument appeared in a non-loop loop. Example of erroneous code:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "let result = while true {\n    if satisfied(i) {\n        break 2 * i; // error: `break` with value from a `while` loop\n    }\n    i += 1;\n};"));

        jd
    }
}
