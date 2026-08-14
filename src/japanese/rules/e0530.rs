use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0530;

impl DiagnosticRule for E0530 {
    fn code(&self) -> &'static str {
        "E0530"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "A binding shadowed something it shouldn't"
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
            "A binding shadowed something it shouldn't. A match arm or a variable has a name that is already used by",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "enum Enum {\n    WithField(i32)\n}\n\nuse Enum::*;\nmatch WithField(1) {\n    WithField => {} // error: missing (_)\n}"));

        jd
    }
}
