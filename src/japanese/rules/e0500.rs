use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0500;

impl DiagnosticRule for E0500 {
    fn code(&self) -> &'static str {
        "E0500"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "A borrowed variable was used by a closure"
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
            "A borrowed variable was used by a closure. Erroneous code example:",
            "Rustの借用規則（不変参照と可変参照の排他性）による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn you_know_nothing(jon_snow: &mut i32) {\n    let nights_watch = &jon_snow;\n    let starks = || {\n        *jon_snow = 3; // error: closure requires unique access to `jon_snow`\n                       //        but it is already borrowed\n    };\n    println!(\"{}\", nights_watch);\n}"));

        jd
    }
}
