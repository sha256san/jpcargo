use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0687;

impl DiagnosticRule for E0687 {
    fn code(&self) -> &'static str {
        "E0687"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "In-band lifetimes cannot be used in fn/Fn syntax"
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
            "In-band lifetimes cannot be used in fn/Fn syntax. Erroneous code examples:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(in_band_lifetimes)]\n\nfn foo(x: fn(&'a u32)) {} // error!\n\nfn bar(x: &Fn(&'a u32)) {} // error!\n\nfn baz(x: fn(&'a u32), y: &'a u32) {} // error!\n\nstruct Foo<'a> { x: &'a u32 }\n\nimpl Foo<'a> {\n    fn bar(&self, x: fn(&'a u32)) {} // error!\n}"));

        jd
    }
}
