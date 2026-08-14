use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0055;

impl DiagnosticRule for E0055 {
    fn code(&self) -> &'static str {
        "E0055"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "During a method call, a value is automatically dereferenced as many times as"
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
            "During a method call, a value is automatically dereferenced as many times as needed to make the value's type match the method's receiver. The catch is that",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![recursion_limit=\"4\"]\n\nstruct Foo;\n\nimpl Foo {\n    fn foo(&self) {}\n}\n\nfn main() {\n    let foo = Foo;\n    let ref_foo = &&&&&Foo;\n\n    // error, reached the recursion limit while auto-dereferencing `&&&&&Foo`\n    ref_foo.foo();\n}"));

        jd
    }
}
