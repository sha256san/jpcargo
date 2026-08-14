use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0226;

impl DiagnosticRule for E0226 {
    fn code(&self) -> &'static str {
        "E0226"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "More than one explicit lifetime bound was used on a trait object"
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
            "More than one explicit lifetime bound was used on a trait object. Example of erroneous code:",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait Foo {}\n\ntype T<'a, 'b> = dyn Foo + 'a + 'b; // error: Trait object `arg` has two\n                                    //        lifetime bound, 'a and 'b."));

        jd
    }
}
