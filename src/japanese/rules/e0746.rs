use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0746;

impl DiagnosticRule for E0746 {
    fn code(&self) -> &'static str {
        "E0746"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "An unboxed trait object was used as a return value"
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
            "An unboxed trait object was used as a return value. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait T {\n    fn bar(&self);\n}\nstruct S(usize);\nimpl T for S {\n    fn bar(&self) {}\n}\n\n// Having the trait `T` as return type is invalid because\n// unboxed trait objects do not have a statically known size:\nfn foo() -> dyn T { // error!\n    S(42)\n}"));

        jd
    }
}
