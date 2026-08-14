use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0195;

impl DiagnosticRule for E0195 {
    fn code(&self) -> &'static str {
        "E0195"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "The lifetime parameters of the method do not match the trait declaration"
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
            "The lifetime parameters of the method do not match the trait declaration. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait Trait {\n    fn bar<'a,'b:'a>(x: &'a str, y: &'b str);\n}\n\nstruct Foo;\n\nimpl Trait for Foo {\n    fn bar<'a,'b>(x: &'a str, y: &'b str) {\n    // error: lifetime parameters or bounds on method `bar`\n    // do not match the trait declaration\n    }\n}"));

        jd
    }
}
