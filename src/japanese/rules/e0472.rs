use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0472;

impl DiagnosticRule for E0472 {
    fn code(&self) -> &'static str {
        "E0472"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "Inline assembly (asm!) is not supported on this target"
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
            "Inline assembly (asm!) is not supported on this target. Example of erroneous code:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "// compile-flags: --target sparc64-unknown-linux-gnu\n#![no_std]\n\nuse core::arch::asm;\n\nfn main() {\n    unsafe {\n        asm!(\"\"); // error: inline assembly is not supported on this target\n    }\n}"));

        jd
    }
}
