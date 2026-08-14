use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0711;

impl DiagnosticRule for E0711 {
    fn code(&self) -> &'static str {
        "E0711"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "Feature declared with conflicting stability requirements"
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
            "Feature declared with conflicting stability requirements. ",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "// NOTE: this attribute is perma-unstable and should *never* be used outside of\n//       stdlib and the compiler.\n#![feature(staged_api)]\n\n#![stable(feature = \"...\", since = \"1.0.0\")]\n\n#[stable(feature = \"foo\", since = \"1.0.0\")]\nfn foo_stable_1_0_0() {}\n\n// error: feature `foo` is declared stable since 1.29.0\n#[stable(feature = \"foo\", since = \"1.29.0\")]\nfn foo_stable_1_29_0() {}\n\n// error: feature `foo` is declared unstable\n#[unstable(feature = \"foo\", issue = \"none\")]\nfn foo_unstable() {}"));

        jd
    }
}
