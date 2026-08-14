use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0094;

impl DiagnosticRule for E0094 {
    fn code(&self) -> &'static str {
        "E0094"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Generic
    }

    fn title(&self) -> &'static str {
        "An invalid number of generic parameters was passed to an intrinsic function"
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
            "An invalid number of generic parameters was passed to an intrinsic function. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(intrinsics)]\n#![allow(internal_features)]\n\n#[rustc_intrinsic]\nfn size_of<T, U>() -> usize; // error: intrinsic has wrong number\n                             //        of type parameters"));

        jd
    }
}
