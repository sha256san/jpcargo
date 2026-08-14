use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0436;

impl DiagnosticRule for E0436 {
    fn code(&self) -> &'static str {
        "E0436"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "The functional record update syntax was used on something other than a struct"
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
            "The functional record update syntax was used on something other than a struct. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "enum PublicationFrequency {\n    Weekly,\n    SemiMonthly { days: (u8, u8), annual_special: bool },\n}\n\nfn one_up_competitor(competitor_frequency: PublicationFrequency)\n                     -> PublicationFrequency {\n    match competitor_frequency {\n        PublicationFrequency::Weekly => PublicationFrequency::SemiMonthly {\n            days: (1, 15), annual_special: false\n        },\n        c @ PublicationFrequency::SemiMonthly{ .. } =>\n            PublicationFrequency::SemiMonthly {\n                annual_special: true, ..c // error: functional record update\n                                          //        syntax requires a struct\n        }\n    }\n}"));

        jd
    }
}
