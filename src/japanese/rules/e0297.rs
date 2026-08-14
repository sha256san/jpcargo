use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0297;

impl DiagnosticRule for E0297 {
    fn code(&self) -> &'static str {
        "E0297"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "Patterns used to bind names must be irrefutable"
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
            "Patterns used to bind names must be irrefutable. That is, they must guarantee that a name will be extracted in all cases. Instead of pattern matching the",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "let xs : Vec<Option<i32>> = vec![Some(1), None];\n\n// This fails because `None` is not covered.\nfor Some(x) in xs {\n    // ...\n}"));

        jd
    }
}
