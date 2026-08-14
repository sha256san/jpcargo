use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0423;

impl DiagnosticRule for E0423 {
    fn code(&self) -> &'static str {
        "E0423"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "An identifier was used like a function name or a value was expected and the"
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
            "An identifier was used like a function name or a value was expected and the identifier exists but it belongs to a different namespace.",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Foo { a: bool };\n\nlet f = Foo();\n// error: expected function, tuple struct or tuple variant, found `Foo`\n// `Foo` is a struct name, but this expression uses it like a function name"));

        jd
    }
}
