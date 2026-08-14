use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0029;

impl DiagnosticRule for E0029 {
    fn code(&self) -> &'static str {
        "E0029"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "Something other than numbers and characters has been used for a range"
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
            "Something other than numbers and characters has been used for a range. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "let string = \"salutations !\";\n\n// The ordering relation for strings cannot be evaluated at compile time,\n// so this doesn't work:\nmatch string {\n    \"hello\" ..= \"world\" => {}\n    _ => {}\n}\n\n// This is a more general version, using a guard:\nmatch string {\n    s if s >= \"hello\" && s <= \"world\" => {}\n    _ => {}\n}"));

        jd
    }
}
