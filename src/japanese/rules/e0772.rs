use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0772;

impl DiagnosticRule for E0772 {
    fn code(&self) -> &'static str {
        "E0772"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "A trait object has some specific lifetime '1, but it was used in a way that"
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
            "A trait object has some specific lifetime '1, but it was used in a way that requires it to have a 'static lifetime.",
            "このエラーコードは古いバージョンの rustc で使用されていましたが、現在は別のエラーコードに統合されたか、非推奨となっています。",
            "最新のRustコンパイラのエラー診断メッセージを参照してください。",
        );

        jd.beginner_tip = Some("※ このエラーコードは古いバージョンの rustc で使われていましたが、現在は非推奨または別のコードに統合されています。".to_string());

        jd.suggestions.push(format!("コード例:\n{}", "trait BooleanLike {}\ntrait Person {}\n\nimpl BooleanLike for bool {}\n\nimpl dyn Person {\n    fn is_cool(&self) -> bool {\n        // hey you, you're pretty cool\n        true\n    }\n}\n\nfn get_is_cool<'p>(person: &'p dyn Person) -> impl BooleanLike {\n    // error: `person` has an anonymous lifetime `'p` but calling\n    //        `print_cool_fn` introduces an implicit `'static` lifetime\n    //        requirement\n    person.is_cool()\n}"));

        jd
    }
}
