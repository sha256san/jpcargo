use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0589;

impl DiagnosticRule for E0589 {
    fn code(&self) -> &'static str {
        "E0589"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "The value of N that was specified for repr(align(N)) was not a power"
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
            "The value of N that was specified for repr(align(N)) was not a power of two, or was greater than 2^29.",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#[repr(align(15))] // error: invalid `repr(align)` attribute: not a power of two\nenum Foo {\n    Bar(u64),\n}"));

        jd
    }
}
