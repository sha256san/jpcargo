use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0539;

impl DiagnosticRule for E0539 {
    fn code(&self) -> &'static str {
        "E0539"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "An invalid meta-item was used inside an attribute"
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
            "An invalid meta-item was used inside an attribute. Erroneous code example:",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "#![feature(staged_api)]\n#![allow(internal_features)]\n#![stable(since = \"1.0.0\", feature = \"test\")]\n\n#[deprecated(note)] // error!\n#[unstable(feature = \"deprecated_fn\", issue = \"123\")]\nfn deprecated() {}\n\n#[unstable(feature = \"unstable_struct\", issue)] // error!\nstruct Unstable;\n\n#[rustc_const_unstable(feature)] // error!\nconst fn unstable_fn() {}\n\n#[stable(feature = \"stable_struct\", since)] // error!\nstruct Stable;\n\n#[rustc_const_stable(feature)] // error!\nconst fn stable_fn() {}"));

        jd
    }
}
