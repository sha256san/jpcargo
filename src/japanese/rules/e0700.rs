use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0700;

impl DiagnosticRule for E0700 {
    fn code(&self) -> &'static str {
        "E0700"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "The impl Trait return type captures lifetime parameters that do not"
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
            "The impl Trait return type captures lifetime parameters that do not appear within the impl Trait itself.",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "use std::cell::Cell;\n\ntrait Trait<'a> { }\n\nimpl<'a, 'b> Trait<'b> for Cell<&'a u32> { }\n\nfn foo<'x, 'y>(x: Cell<&'x u32>) -> impl Trait<'y>\nwhere 'x: 'y\n{\n    x\n}"));

        jd
    }
}
