use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0060;

impl DiagnosticRule for E0060 {
    fn code(&self) -> &'static str {
        "E0060"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "External C functions are allowed to be variadic"
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
            "External C functions are allowed to be variadic. However, a variadic function takes a minimum number of arguments. For example, consider C's variadic printf",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "use std::os::raw::{c_char, c_int};\n\nextern \"C\" {\n    fn printf(_: *const c_char, ...) -> c_int;\n}\n\nunsafe { printf(); } // error!"));

        jd
    }
}
