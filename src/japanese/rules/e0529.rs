use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0529;

impl DiagnosticRule for E0529 {
    fn code(&self) -> &'static str {
        "E0529"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "An array or slice pattern was matched against some other type"
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
            "An array or slice pattern was matched against some other type. Example of erroneous code:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "let r: f32 = 1.0;\nmatch r {\n    [a, b] => { // error: expected an array or slice, found `f32`\n        println!(\"a={}, b={}\", a, b);\n    }\n}"));

        jd
    }
}
