use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0713;

impl DiagnosticRule for E0713 {
    fn code(&self) -> &'static str {
        "E0713"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "This error occurs when an attempt is made to borrow state past the end of the"
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
            "This error occurs when an attempt is made to borrow state past the end of the lifetime of a type that implements the Drop trait.",
            "Rustの借用規則（不変参照と可変参照の排他性）による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "pub struct S<'a> { data: &'a mut String }\n\nimpl<'a> Drop for S<'a> {\n    fn drop(&mut self) { self.data.push_str(\"being dropped\"); }\n}\n\nfn demo<'a>(s: S<'a>) -> &'a mut String { let p = &mut *s.data; p }"));

        jd
    }
}
