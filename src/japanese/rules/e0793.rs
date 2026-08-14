use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0793;

impl DiagnosticRule for E0793 {
    fn code(&self) -> &'static str {
        "E0793"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "An unaligned reference to a field of a [packed] struct or union was created"
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
            "An unaligned reference to a field of a [packed] struct or union was created. The #[repr(packed)] attribute removes padding between fields, which can",
            "Rustコンパイラの安全性検査・型システム・構文規則により検出されました。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#[repr(packed)]\npub struct Foo {\n    field1: u64,\n    field2: u8,\n}\n\nunsafe {\n    let foo = Foo { field1: 0, field2: 0 };\n    // Accessing the field directly is fine.\n    let val = foo.field1;\n    // A reference to a packed field causes a error.\n    let val = &foo.field1; // ERROR\n    // An implicit `&` is added in format strings, causing the same error.\n    println!(\"{}\", foo.field1); // ERROR\n}"));

        jd
    }
}
