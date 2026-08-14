use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0109;

impl DiagnosticRule for E0109 {
    fn code(&self) -> &'static str {
        "E0109"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "You tried to provide a generic argument to a type which doesn't need it"
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
            "You tried to provide a generic argument to a type which doesn't need it. Erroneous code example:",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "type X = u32<i32>; // error: type arguments are not allowed for this type\ntype Y = bool<'static>; // error: lifetime parameters are not allowed on\n                        //        this type"));

        jd
    }
}
