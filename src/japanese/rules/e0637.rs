use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0637;

impl DiagnosticRule for E0637 {
    fn code(&self) -> &'static str {
        "E0637"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "'_ lifetime name or &T without an explicit lifetime name has been used"
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
            "'_ lifetime name or &T without an explicit lifetime name has been used in an illegal place.",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "fn underscore_lifetime<'_>(str1: &'_ str, str2: &'_ str) -> &'_ str {\n                     //^^ `'_` is a reserved lifetime name\n    if str1.len() > str2.len() {\n        str1\n    } else {\n        str2\n    }\n}\n\nfn without_explicit_lifetime<T>()\nwhere\n    T: Iterator<Item = &u32>,\n                     //^ `&` without an explicit lifetime name\n{\n}\n\nfn without_hrtb<T>()\nwhere\n    T: Into<&u32>,\n          //^ `&` without an explicit lifetime name\n{\n}"));

        jd
    }
}
