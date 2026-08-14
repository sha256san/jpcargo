use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0387;

impl DiagnosticRule for E0387 {
    fn code(&self) -> &'static str {
        "E0387"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "This error occurs when an attempt is made to mutate or mutably reference data"
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
            "This error occurs when an attempt is made to mutate or mutably reference data that a closure has captured immutably.",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "// Accepts a function or a closure that captures its environment immutably.\n// Closures passed to foo will not be able to mutate their closed-over state.\nfn foo<F: Fn()>(f: F) { }\n\n// Attempts to mutate closed-over data. Error message reads:\n// `cannot assign to data in a captured outer variable...`\nfn mutable() {\n    let mut x = 0u32;\n    foo(|| x = 2);\n}\n\n// Attempts to take a mutable reference to closed-over data. Error message\n// reads: `cannot borrow data mutably in a captured outer variable...`\nfn mut_addr() {\n    let mut x = 0u32;\n    foo(|| { let y = &mut x; });\n}"));

        jd
    }
}
