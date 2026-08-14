use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0439;

impl DiagnosticRule for E0439 {
    fn code(&self) -> &'static str {
        "E0439"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Function
    }

    fn title(&self) -> &'static str {
        "The length of the platform-intrinsic function simd_shuffle wasn't specified"
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
            "The length of the platform-intrinsic function simd_shuffle wasn't specified. Erroneous code example:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(platform_intrinsics)]\n\nextern \"platform-intrinsic\" {\n    fn simd_shuffle<A,B>(a: A, b: A, c: [u32; 8]) -> B;\n    // error: invalid `simd_shuffle`, needs length: `simd_shuffle`\n}"));

        jd
    }
}
