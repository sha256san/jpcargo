use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0445;

impl DiagnosticRule for E0445 {
    fn code(&self) -> &'static str {
        "E0445"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "A private trait was used on a public type parameter bound"
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
            "A private trait was used on a public type parameter bound. Previously erroneous code examples:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "trait Foo {\n    fn dummy(&self) { }\n}\n\npub trait Bar : Foo {} // error: private trait in public interface\npub struct Bar2<T: Foo>(pub T); // same error\npub fn foo<T: Foo> (t: T) {} // same error\n\nfn main() {}"));

        jd
    }
}
