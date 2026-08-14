use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0582;

impl DiagnosticRule for E0582 {
    fn code(&self) -> &'static str {
        "E0582"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "A lifetime is only present in an associated-type binding, and not in the input"
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
            "A lifetime is only present in an associated-type binding, and not in the input types to the trait.",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn bar<F>(t: F)\n    // No type can satisfy this requirement, since `'a` does not\n    // appear in any of the input types (here, `i32`):\n    where F: for<'a> Fn(i32) -> Option<&'a i32>\n{\n}\n\nfn main() { }"));

        jd
    }
}
