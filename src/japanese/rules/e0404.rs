use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0404;

impl DiagnosticRule for E0404 {
    fn code(&self) -> &'static str {
        "E0404"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "A type that is not a trait was used in a trait position, such as a bound"
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
            "A type that is not a trait was used in a trait position, such as a bound or impl.",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Foo;\nstruct Bar;\n\nimpl Foo for Bar {} // error: `Foo` is not a trait\nfn baz<T: Foo>(t: T) {} // error: `Foo` is not a trait"));

        jd
    }
}
