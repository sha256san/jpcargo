use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0377;

impl DiagnosticRule for E0377 {
    fn code(&self) -> &'static str {
        "E0377"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "CoerceUnsized or DispatchFromDyn may only be implemented between structs"
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
            "CoerceUnsized or DispatchFromDyn may only be implemented between structs of the same type.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(coerce_unsized)]\nuse std::ops::CoerceUnsized;\n\npub struct Foo<T: ?Sized> {\n    field_with_unsized_type: T,\n}\n\npub struct Bar<T: ?Sized> {\n    field_with_unsized_type: T,\n}\n\n// error: the trait `CoerceUnsized` may only be implemented for a coercion\n//        between structures with the same definition\nimpl<T, U> CoerceUnsized<Bar<U>> for Foo<T> where T: CoerceUnsized<U> {}"));

        jd
    }
}
