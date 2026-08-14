use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0312;

impl DiagnosticRule for E0312 {
    fn code(&self) -> &'static str {
        "E0312"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "Reference's lifetime of borrowed content doesn't match the expected lifetime"
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
            "Reference's lifetime of borrowed content doesn't match the expected lifetime. Erroneous code example:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "pub fn opt_str<'a>(maybestr: &'a Option<String>) -> &'static str {\n    if maybestr.is_none() {\n        \"(none)\"\n    } else {\n        let s: &'a str = maybestr.as_ref().unwrap();\n        s  // Invalid lifetime!\n    }\n}"));

        jd
    }
}
