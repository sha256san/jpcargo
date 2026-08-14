use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0070;

impl DiagnosticRule for E0070 {
    fn code(&self) -> &'static str {
        "E0070"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Mutability
    }

    fn title(&self) -> &'static str {
        "An assignment operator was used on a non-place expression"
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
            "An assignment operator was used on a non-place expression. Erroneous code examples:",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct SomeStruct {\n    x: i32,\n    y: i32,\n}\n\nconst SOME_CONST: i32 = 12;\n\nfn some_other_func() {}\n\nfn some_function() {\n    SOME_CONST = 14; // error: a constant value cannot be changed!\n    1 = 3; // error: 1 isn't a valid place!\n    some_other_func() = 4; // error: we cannot assign value to a function!\n    SomeStruct::x = 12; // error: SomeStruct a structure name but it is used\n                        //        like a variable!\n}"));

        jd
    }
}
