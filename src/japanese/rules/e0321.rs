use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0321;

impl DiagnosticRule for E0321 {
    fn code(&self) -> &'static str {
        "E0321"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "A cross-crate opt-out trait was implemented on something which wasn't a struct"
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
            "A cross-crate opt-out trait was implemented on something which wasn't a struct or enum type.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(auto_traits)]\n\nstruct Foo;\n\nimpl !Sync for Foo {}\n\nunsafe impl Send for &'static Foo {}\n// error: cross-crate traits with a default impl, like `core::marker::Send`,\n//        can only be implemented for a struct/enum type, not\n//        `&'static Foo`"));

        jd
    }
}
