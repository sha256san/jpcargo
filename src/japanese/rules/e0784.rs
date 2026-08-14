use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0784;

impl DiagnosticRule for E0784 {
    fn code(&self) -> &'static str {
        "E0784"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "A union expression does not have exactly one field"
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
            "A union expression does not have exactly one field. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "union Bird {\n    pigeon: u8,\n    turtledove: u16,\n}\n\nlet bird = Bird {}; // error\nlet bird = Bird { pigeon: 0, turtledove: 1 }; // error"));

        jd
    }
}
