use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0449;

impl DiagnosticRule for E0449 {
    fn code(&self) -> &'static str {
        "E0449"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "A visibility qualifier was used where one is not permitted"
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
            "A visibility qualifier was used where one is not permitted. Visibility qualifiers are not permitted on enum variants, trait items, impl blocks, and",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Bar;\n\ntrait Foo {\n    fn foo();\n}\n\nenum Baz {\n    pub Qux, // error: visibility qualifiers are not permitted here\n}\n\npub impl Bar {} // error: visibility qualifiers are not permitted here\n\npub impl Foo for Bar { // error: visibility qualifiers are not permitted here\n    pub fn foo() {} // error: visibility qualifiers are not permitted here\n}"));

        jd
    }
}
