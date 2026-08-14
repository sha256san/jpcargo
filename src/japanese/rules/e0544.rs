use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0544;

impl DiagnosticRule for E0544 {
    fn code(&self) -> &'static str {
        "E0544"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Operator
    }

    fn title(&self) -> &'static str {
        "Multiple stability attributes were declared on the same item"
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
            "Multiple stability attributes were declared on the same item. Erroneous code example:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(staged_api)]\n#![allow(internal_features)]\n#![stable(since = \"1.0.0\", feature = \"rust1\")]\n\n#[stable(feature = \"rust1\", since = \"1.0.0\")]\n#[stable(feature = \"test\", since = \"2.0.0\")] // invalid\nfn foo() {}"));

        jd
    }
}
