use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0798;

impl DiagnosticRule for E0798 {
    fn code(&self) -> &'static str {
        "E0798"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Generic
    }

    fn title(&self) -> &'static str {
        "Functions marked as cmse-nonsecure-call place restrictions on their"
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
            "Functions marked as cmse-nonsecure-call place restrictions on their inputs and outputs.",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(abi_cmse_nonsecure_call)]\n\n#[no_mangle]\npub fn test(\n    f: extern \"cmse-nonsecure-call\" fn(u32, u32, u32, u32, u32) -> u32,\n) -> u32 {\n    f(1, 2, 3, 4, 5)\n}"));

        jd
    }
}
