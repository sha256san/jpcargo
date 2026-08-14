use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0061;

impl DiagnosticRule for E0061 {
    fn code(&self) -> &'static str {
        "E0061"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Function
    }

    fn title(&self) -> &'static str {
        "関数の引数の個数が一致していません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let count_re = Regex::new(r"this function takes (?P<exp>\d+) argument[s]? but (?P<found>\d+) argument[s]? (were|was) supplied").unwrap();
        let (expected, found) = if let Some(caps) = count_re.captures(&diag.message) {
            (
                caps.name("exp").map(|m| m.as_str()).unwrap_or("指定の"),
                caps.name("found").map(|m| m.as_str()).unwrap_or("異なる"),
            )
        } else {
            ("指定の", "異なる")
        };

        let summary = format!(
            "この関数は {} 個の引数を必要としますが、呼び出し時に {} 個しか渡されていません。",
            expected, found
        );

        let reason = "Rust では関数の引数の個数と型が完全一致している必要があります。可変長引数やデフォルト引数はサポートされていません。";

        let solution = format!(
            "関数の定義に合わせて、過不足なく {} 個の引数を渡してください。",
            expected
        );

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "関数呼び出し時に渡された引数の個数が、関数の定義と合致しない場合に発生します。",
            "引数の渡し忘れや渡しすぎが原因です。",
            "関数定義で要求されている通りの個数の引数を渡してください。",
        )
    }
}
