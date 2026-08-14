use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0586;

impl DiagnosticRule for E0586 {
    fn code(&self) -> &'static str {
        "E0586"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "An inclusive range was used with no end"
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
            "An inclusive range was used with no end. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn main() {\n    let tmp = vec![0, 1, 2, 3, 4, 4, 3, 3, 2, 1];\n    let x = &tmp[1..=]; // error: inclusive range was used with no end\n}"));

        jd
    }
}
