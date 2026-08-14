use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0591;

impl DiagnosticRule for E0591 {
    fn code(&self) -> &'static str {
        "E0591"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "Per [RFC 401][rfc401], if you have a function declaration foo:"
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
            "Per [RFC 401][rfc401], if you have a function declaration foo: ",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct S;\n\n// For the purposes of this explanation, all of these\n// different kinds of `fn` declarations are equivalent:\n\nfn foo(x: S) { /* ... */ }\nextern \"C\" {\n    fn foo(x: S);\n}\nimpl S {\n    fn foo(self) { /* ... */ }\n}"));

        jd
    }
}
