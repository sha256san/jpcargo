use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0501;

impl DiagnosticRule for E0501 {
    fn code(&self) -> &'static str {
        "E0501"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "A mutable variable is used but it is already captured by a closure"
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
            "A mutable variable is used but it is already captured by a closure. Erroneous code example:",
            "Rustの借用規則（不変参照と可変参照の排他性）による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn inside_closure(x: &mut i32) {\n    // Actions which require unique access\n}\n\nfn outside_closure(x: &mut i32) {\n    // Actions which require unique access\n}\n\nfn foo(a: &mut i32) {\n    let mut bar = || {\n        inside_closure(a)\n    };\n    outside_closure(a); // error: cannot borrow `*a` as mutable because previous\n                        //        closure requires unique access.\n    bar();\n}"));

        jd
    }
}
