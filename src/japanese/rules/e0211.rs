use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0211;

impl DiagnosticRule for E0211 {
    fn code(&self) -> &'static str {
        "E0211"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "You used a function or type which doesn't fit the requirements for where it was"
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
            "You used a function or type which doesn't fit the requirements for where it was used. Erroneous code examples:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(intrinsics)]\n#![allow(internal_features)]\n\n#[rustc_intrinsic]\nunsafe fn unreachable(); // error: intrinsic has wrong type\n\n// or:\n\nfn main() -> i32 { 0 }\n// error: main function expects type: `fn() {main}`: expected (), found i32\n\n// or:\n\nlet x = 1u8;\nmatch x {\n    0u8..=3i8 => (),\n    // error: mismatched types in range: expected u8, found i8\n    _ => ()\n}\n\n// or:\n\nuse std::rc::Rc;\nstruct Foo;\n\nimpl Foo {\n    fn x(self: Rc<Foo>) {}\n    // error: mismatched self type: expected `Foo`: expected struct\n    //        `Foo`, found struct `alloc::rc::Rc`\n}"));

        jd
    }
}
