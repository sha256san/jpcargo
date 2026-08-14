use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0424;

impl DiagnosticRule for E0424 {
    fn code(&self) -> &'static str {
        "E0424"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "The self keyword was used inside of an associated function without a \"self"
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
            "The self keyword was used inside of an associated function without a \"self receiver\" parameter.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Foo;\n\nimpl Foo {\n    // `bar` is a method, because it has a receiver parameter.\n    fn bar(&self) {}\n\n    // `foo` is not a method, because it has no receiver parameter.\n    fn foo() {\n        self.bar(); // error: `self` value is a keyword only available in\n                    //        methods with a `self` parameter\n    }\n}"));

        jd
    }
}
