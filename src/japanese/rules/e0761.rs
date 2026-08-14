use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0761;

impl DiagnosticRule for E0761 {
    fn code(&self) -> &'static str {
        "E0761"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "Multiple candidate files were found for an out-of-line module"
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
            "Multiple candidate files were found for an out-of-line module. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "// file: ambiguous_module/mod.rs\n\nfn foo() {}\n\n// file: ambiguous_module.rs\n\nfn foo() {}\n\n// file: lib.rs\n\nmod ambiguous_module; // error: file for module `ambiguous_module`\n                      // found at both ambiguous_module.rs and\n                      // ambiguous_module/mod.rs"));

        jd
    }
}
