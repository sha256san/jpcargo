use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0399;

impl DiagnosticRule for E0399 {
    fn code(&self) -> &'static str {
        "E0399"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "You implemented a trait, overriding one or more of its associated types but did"
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
            "You implemented a trait, overriding one or more of its associated types but did not reimplement its default methods.",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(associated_type_defaults)]\n\npub trait Foo {\n    type Assoc = u8;\n    fn bar(&self) {}\n}\n\nimpl Foo for i32 {\n    // error - the following trait items need to be reimplemented as\n    //         `Assoc` was overridden: `bar`\n    type Assoc = i32;\n}"));

        jd
    }
}
