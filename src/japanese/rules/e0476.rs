use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0476;

impl DiagnosticRule for E0476 {
    fn code(&self) -> &'static str {
        "E0476"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "The coerced type does not outlive the value being coerced to"
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
            "The coerced type does not outlive the value being coerced to. Example of erroneous code:",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(coerce_unsized)]\n#![feature(unsize)]\n\nuse std::marker::Unsize;\nuse std::ops::CoerceUnsized;\n\n// error: lifetime of the source pointer does not outlive lifetime bound of the\n//        object type\nimpl<'a, 'b, T, S> CoerceUnsized<&'a T> for &'b S where S: Unsize<T> {}"));

        jd
    }
}
