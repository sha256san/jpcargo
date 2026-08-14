use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0201;

impl DiagnosticRule for E0201 {
    fn code(&self) -> &'static str {
        "E0201"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "Two associated items (like methods, associated types, associated functions,"
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
            "Two associated items (like methods, associated types, associated functions, etc.) were defined with the same identifier.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Foo(u8);\n\nimpl Foo {\n    fn bar(&self) -> bool { self.0 > 5 }\n    fn bar() {} // error: duplicate associated function\n}\n\ntrait Baz {\n    type Quux;\n    fn baz(&self) -> bool;\n}\n\nimpl Baz for Foo {\n    type Quux = u32;\n\n    fn baz(&self) -> bool { true }\n\n    // error: duplicate method\n    fn baz(&self) -> bool { self.0 > 5 }\n\n    // error: duplicate associated type\n    type Quux = u32;\n}"));

        jd
    }
}
