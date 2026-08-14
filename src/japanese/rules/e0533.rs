use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0533;

impl DiagnosticRule for E0533 {
    fn code(&self) -> &'static str {
        "E0533"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "An item which isn't a unit struct, a variant, nor a constant has been used as a"
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
            "An item which isn't a unit struct, a variant, nor a constant has been used as a match pattern.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Tortoise;\n\nimpl Tortoise {\n    fn turtle(&self) -> u32 { 0 }\n}\n\nmatch 0u32 {\n    Tortoise::turtle => {} // Error!\n    _ => {}\n}\nif let Tortoise::turtle = 0u32 {} // Same error!"));

        jd
    }
}
