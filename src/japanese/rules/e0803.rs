use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0803;

impl DiagnosticRule for E0803 {
    fn code(&self) -> &'static str {
        "E0803"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "A trait implementation returns a reference without an"
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
            "A trait implementation returns a reference without an explicit lifetime linking it to self.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait DataAccess<T> {\n    fn get_ref(&self) -> T;\n}\n\nstruct Container<'a> {\n    value: &'a f64,\n}\n\n// Attempting to implement reference return\nimpl<'a> DataAccess<&f64> for Container<'a> {\n    fn get_ref(&self) -> &f64 { // Error: Lifetime mismatch\n        self.value\n    }\n}"));

        jd
    }
}
