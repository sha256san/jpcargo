use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0383;

impl DiagnosticRule for E0383 {
    fn code(&self) -> &'static str {
        "E0383"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "This error occurs when an attempt is made to partially reinitialize a"
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
            "This error occurs when an attempt is made to partially reinitialize a structure that is currently uninitialized.",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "struct Foo {\n    a: u32,\n}\nimpl Drop for Foo {\n    fn drop(&mut self) { /* ... */ }\n}\n\nlet mut x = Foo { a: 1 };\ndrop(x); // `x` is now uninitialized\nx.a = 2; // error, partial reinitialization of uninitialized structure `t`"));

        jd
    }
}
