use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0715;

impl DiagnosticRule for E0715 {
    fn code(&self) -> &'static str {
        "E0715"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "An impl for a #[marker] trait tried to override an associated item"
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
            "An impl for a #[marker] trait tried to override an associated item. Erroneous code example:",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(marker_trait_attr)]\n\n#[marker]\ntrait Marker {\n    const N: usize = 0;\n    fn do_something() {}\n}\n\nstruct OverrideConst;\nimpl Marker for OverrideConst { // error!\n    const N: usize = 1;\n}"));

        jd
    }
}
