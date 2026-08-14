use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0378;

impl DiagnosticRule for E0378 {
    fn code(&self) -> &'static str {
        "E0378"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "The DispatchFromDyn trait was implemented on something which is not a pointer"
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
            "The DispatchFromDyn trait was implemented on something which is not a pointer or a newtype wrapper around a pointer.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(dispatch_from_dyn)]\nuse std::ops::DispatchFromDyn;\n\nstruct WrapperExtraField<T> {\n    ptr: T,\n    extra_stuff: i32,\n}\n\nimpl<T, U> DispatchFromDyn<WrapperExtraField<U>> for WrapperExtraField<T>\nwhere\n    T: DispatchFromDyn<U>,\n{}"));

        jd
    }
}
