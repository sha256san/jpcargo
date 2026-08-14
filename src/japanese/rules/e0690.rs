use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0690;

impl DiagnosticRule for E0690 {
    fn code(&self) -> &'static str {
        "E0690"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Generic
    }

    fn title(&self) -> &'static str {
        "A struct with the representation hint repr(transparent) had two or more fields"
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
            "A struct with the representation hint repr(transparent) had two or more fields that were not guaranteed to be zero-sized.",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#[repr(transparent)]\nstruct LengthWithUnit<U> { // error: transparent struct needs at most one\n    value: f32,            //        non-zero-sized field, but has 2\n    unit: U,\n}"));

        jd
    }
}
