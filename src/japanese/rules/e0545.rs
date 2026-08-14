use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0545;

impl DiagnosticRule for E0545 {
    fn code(&self) -> &'static str {
        "E0545"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "The issue value is incorrect in a stability attribute"
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
            "The issue value is incorrect in a stability attribute. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(staged_api)]\n#![allow(internal_features)]\n#![stable(since = \"1.0.0\", feature = \"test\")]\n\n#[unstable(feature = \"_unstable_fn\", issue = \"0\")] // invalid\nfn _unstable_fn() {}\n\n#[rustc_const_unstable(feature = \"_unstable_const_fn\", issue = \"0\")] // invalid\nconst fn _unstable_const_fn() {}"));

        jd
    }
}
