use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0389;

impl DiagnosticRule for E0389 {
    fn code(&self) -> &'static str {
        "E0389"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "An attempt was made to mutate data using a non-mutable reference"
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
            "An attempt was made to mutate data using a non-mutable reference. This commonly occurs when attempting to assign to a non-mutable reference of a",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "struct FancyNum {\n    num: u8,\n}\n\nfn main() {\n    let mut fancy = FancyNum{ num: 5 };\n    let fancy_ref = &(&mut fancy);\n    fancy_ref.num = 6; // error: cannot assign to data in a `&` reference\n    println!(\"{}\", fancy_ref.num);\n}"));

        jd
    }
}
