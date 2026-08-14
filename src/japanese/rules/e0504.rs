use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0504;

impl DiagnosticRule for E0504 {
    fn code(&self) -> &'static str {
        "E0504"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "This error occurs when an attempt is made to move a borrowed variable into a"
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
            "This error occurs when an attempt is made to move a borrowed variable into a closure.",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "struct FancyNum {\n    num: u8,\n}\n\nfn main() {\n    let fancy_num = FancyNum { num: 5 };\n    let fancy_ref = &fancy_num;\n\n    let x = move || {\n        println!(\"child function: {}\", fancy_num.num);\n        // error: cannot move `fancy_num` into closure because it is borrowed\n    };\n\n    x();\n    println!(\"main function: {}\", fancy_ref.num);\n}"));

        jd
    }
}
