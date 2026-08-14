use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0790;

impl DiagnosticRule for E0790 {
    fn code(&self) -> &'static str {
        "E0790"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "You need to specify a specific implementation of the trait in order to call the"
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
            "You need to specify a specific implementation of the trait in order to call the method.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait Coroutine {\n    fn create() -> u32;\n}\n\nstruct Impl;\n\nimpl Coroutine for Impl {\n    fn create() -> u32 { 1 }\n}\n\nstruct AnotherImpl;\n\nimpl Coroutine for AnotherImpl {\n    fn create() -> u32 { 2 }\n}\n\nlet cont: u32 = Coroutine::create();\n// error, impossible to choose one of Coroutine trait implementation\n// Should it be Impl or AnotherImpl, maybe something else?"));

        jd
    }
}
