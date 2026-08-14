use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0657;

impl DiagnosticRule for E0657 {
    fn code(&self) -> &'static str {
        "E0657"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "An impl Trait captured a higher-ranked lifetime, which is not supported"
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
            "An impl Trait captured a higher-ranked lifetime, which is not supported. Currently, impl Trait types are only allowed to capture lifetimes from",
            "Rustの借用規則（不変参照と可変参照の排他性）による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait BorrowInto<'a> {\n    type Target;\n\n    fn borrow_into(&'a self) -> Self::Target;\n}\n\nimpl<'a> BorrowInto<'a> for () {\n    type Target = &'a ();\n\n    fn borrow_into(&'a self) -> Self::Target {\n        self\n    }\n}\n\nfn opaque() -> impl for<'a> BorrowInto<'a, Target = impl Sized + 'a> {\n    ()\n}"));

        jd
    }
}
