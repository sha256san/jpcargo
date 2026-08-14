use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0796;

impl DiagnosticRule for E0796 {
    fn code(&self) -> &'static str {
        "E0796"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "You have created a reference to a mutable static"
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
            "You have created a reference to a mutable static. Erroneous code example:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "static mut X: i32 = 23;\nfn work() {\n  let _val = unsafe { X };\n}\nlet x_ref = unsafe { &mut X };\nwork();\n// The next line has Undefined Behavior!\n// `x_ref` is a mutable reference and allows no aliases,\n// but `work` has been reading the reference between\n// the moment `x_ref` was created and when it was used.\n// This violates the uniqueness of `x_ref`.\n*x_ref = 42;"));

        jd
    }
}
