use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0370;

impl DiagnosticRule for E0370 {
    fn code(&self) -> &'static str {
        "E0370"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "The maximum value of an enum was reached, so it cannot be automatically"
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
            "The maximum value of an enum was reached, so it cannot be automatically set in the next enum value.",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#[repr(i64)]\nenum Foo {\n    X = 0x7fffffffffffffff,\n    Y, // error: enum discriminant overflowed on value after\n       //        9223372036854775807: i64; set explicitly via\n       //        Y = -9223372036854775808 if that is desired outcome\n}"));

        jd
    }
}
