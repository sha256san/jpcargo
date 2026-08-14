use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0375;

impl DiagnosticRule for E0375 {
    fn code(&self) -> &'static str {
        "E0375"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "CoerceUnsized or DispatchFromDyn was implemented on a struct which contains"
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
            "CoerceUnsized or DispatchFromDyn was implemented on a struct which contains more than one field that is being unsized.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(coerce_unsized)]\nuse std::ops::CoerceUnsized;\n\nstruct Foo<T: ?Sized, U: ?Sized> {\n    a: i32,\n    b: T,\n    c: U,\n}\n\n// error: Struct `Foo` has more than one unsized field.\nimpl<T, U> CoerceUnsized<Foo<U, T>> for Foo<T, U> {}"));

        jd
    }
}
