use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0434;

impl DiagnosticRule for E0434 {
    fn code(&self) -> &'static str {
        "E0434"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Function
    }

    fn title(&self) -> &'static str {
        "A variable used inside an inner function comes from a dynamic environment"
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
            "A variable used inside an inner function comes from a dynamic environment. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn foo() {\n    let y = 5;\n    fn bar() -> u32 {\n        y // error: can't capture dynamic environment in a fn item; use the\n          //        || { ... } closure form instead.\n    }\n}"));

        jd
    }
}
