use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0409;

impl DiagnosticRule for E0409 {
    fn code(&self) -> &'static str {
        "E0409"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "An \"or\" pattern was used where the variable bindings are not consistently bound"
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
            "An \"or\" pattern was used where the variable bindings are not consistently bound across patterns.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "let x = (0, 2);\nmatch x {\n    (0, ref y) | (y, 0) => { /* use y */} // error: variable `y` is bound with\n                                          //        different mode in pattern #2\n                                          //        than in pattern #1\n    _ => ()\n}"));

        jd
    }
}
