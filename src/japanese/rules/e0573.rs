use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0573;

impl DiagnosticRule for E0573 {
    fn code(&self) -> &'static str {
        "E0573"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "Something other than a type has been used when one was expected"
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
            "Something other than a type has been used when one was expected. Erroneous code examples:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "enum Dragon {\n    Born,\n}\n\nfn oblivion() -> Dragon::Born { // error!\n    Dragon::Born\n}\n\nconst HOBBIT: u32 = 2;\nimpl HOBBIT {} // error!\n\nenum Wizard {\n    Gandalf,\n    Saruman,\n}\n\ntrait Isengard {\n    fn wizard(_: Wizard::Saruman); // error!\n}"));

        jd
    }
}
