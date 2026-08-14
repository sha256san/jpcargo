use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0034;

impl DiagnosticRule for E0034 {
    fn code(&self) -> &'static str {
        "E0034"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "The compiler doesn't know what method to call because more than one method"
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
            "The compiler doesn't know what method to call because more than one method has the same prototype.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Test;\n\ntrait Trait1 {\n    fn foo();\n}\n\ntrait Trait2 {\n    fn foo();\n}\n\nimpl Trait1 for Test { fn foo() {} }\nimpl Trait2 for Test { fn foo() {} }\n\nfn main() {\n    Test::foo() // error, which foo() to call?\n}"));

        jd
    }
}
