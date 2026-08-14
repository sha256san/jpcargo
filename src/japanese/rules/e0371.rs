use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0371;

impl DiagnosticRule for E0371 {
    fn code(&self) -> &'static str {
        "E0371"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "A trait was implemented on another which already automatically implemented it"
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
            "A trait was implemented on another which already automatically implemented it. Erroneous code examples:",
            "要求されているTrait境界やコヒーレンス（孤児規則）の制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "trait Foo { fn foo(&self) { } }\ntrait Bar: Foo { }\ntrait Baz: Bar { }\n\nimpl Bar for Baz { } // error, `Baz` implements `Bar` by definition\nimpl Foo for Baz { } // error, `Baz` implements `Bar` which implements `Foo`\nimpl Baz for Baz { } // error, `Baz` (trivially) implements `Baz`\nimpl Baz for Bar { } // Note: This is OK"));

        jd
    }
}
