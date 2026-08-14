use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0023;

impl DiagnosticRule for E0023 {
    fn code(&self) -> &'static str {
        "E0023"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "A pattern attempted to extract an incorrect number of fields from a variant"
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
            "A pattern attempted to extract an incorrect number of fields from a variant. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "enum Fruit {\n    Apple(String, String),\n    Pear(u32),\n}\n\nlet x = Fruit::Apple(String::new(), String::new());\n\nmatch x {\n    Fruit::Apple(a) => {}, // error!\n    _ => {}\n}"));

        jd
    }
}
