use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0053;

impl DiagnosticRule for E0053 {
    fn code(&self) -> &'static str {
        "E0053"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Mutability
    }

    fn title(&self) -> &'static str {
        "The parameters of any trait method must match between a trait implementation"
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
            "The parameters of any trait method must match between a trait implementation and the trait definition.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait Foo {\n    fn foo(x: u16);\n    fn bar(&self);\n}\n\nstruct Bar;\n\nimpl Foo for Bar {\n    // error, expected u16, found i16\n    fn foo(x: i16) { }\n\n    // error, types differ in mutability\n    fn bar(&mut self) { }\n}"));

        jd
    }
}
