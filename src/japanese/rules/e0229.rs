use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0229;

impl DiagnosticRule for E0229 {
    fn code(&self) -> &'static str {
        "E0229"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "An associated item constraint was written in an unexpected context"
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
            "An associated item constraint was written in an unexpected context. Erroneous code example:",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "pub trait Foo {\n    type A;\n    fn boo(&self) -> <Self as Foo>::A;\n}\n\nstruct Bar;\n\nimpl Foo for isize {\n    type A = usize;\n    fn boo(&self) -> usize { 42 }\n}\n\nfn baz<I>(x: &<I as Foo<A = Bar>>::A) {}\n// error: associated item constraint are not allowed here"));

        jd
    }
}
