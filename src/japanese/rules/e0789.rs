use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0789;

impl DiagnosticRule for E0789 {
    fn code(&self) -> &'static str {
        "E0789"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "The internal rustc_allowed_through_unstable_modules attribute must be used"
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
            "The internal rustc_allowed_through_unstable_modules attribute must be used on an item with a stable attribute.",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "// NOTE: both of these attributes are perma-unstable and should *never* be\n//       used outside of the compiler and standard library.\n#![feature(rustc_attrs)]\n#![feature(staged_api)]\n#![allow(internal_features)]\n\n#![unstable(feature = \"foo_module\", reason = \"...\", issue = \"123\")]\n\n#[rustc_allowed_through_unstable_modules = \"deprecation message\"]\n// #[stable(feature = \"foo\", since = \"1.0\")]\nstruct Foo;\n// ^^^ error: `rustc_allowed_through_unstable_modules` attribute must be\n//            paired with a `stable` attribute"));

        jd
    }
}
