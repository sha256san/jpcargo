use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0503;

impl DiagnosticRule for E0503 {
    fn code(&self) -> &'static str {
        "E0503"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "A value was used after it was mutably borrowed"
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
            "A value was used after it was mutably borrowed. Erroneous code example:",
            "Rustの借用規則（不変参照と可変参照の排他性）による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn main() {\n    let mut value = 3;\n    // Create a mutable borrow of `value`.\n    let borrow = &mut value;\n    let _sum = value + 1; // error: cannot use `value` because\n                          //        it was mutably borrowed\n    println!(\"{}\", borrow);\n}"));

        jd
    }
}
