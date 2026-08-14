use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0369;

impl DiagnosticRule for E0369 {
    fn code(&self) -> &'static str {
        "E0369"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Operator
    }

    fn title(&self) -> &'static str {
        "二項演算子（+, -, *, == 等）をサポートしていない型に適用しようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let _op_re = Regex::new(r"cannot (add|subtract|multiply|divide|compare) `(?P<t1>[^`]+)` (to|and|by) `(?P<t2>[^`]+)`|binary operation `(?P<op>[^`]+)` cannot be applied to type `(?P<t>[^`]+)`").unwrap();
        let summary = "指定された型には該当の二項演算子（`+`, `-`, `*`, `==` など）が実装されていません（例: `&str - &str` など）。";

        let reason = "Rust ではすべての演算子はトレイト（`std::ops::Add`, `Sub`, `std::cmp::PartialEq` 等）として定義されています。対応する演算子トレイトが実装されていない型同士で計算や比較を行うことはできません。";
        let solution = "1. 文字列の結合には `format!(\"{}{}\", a, b)` や `+` (`String` + `&str`) を使用してください。\n2. 自作構造体の場合は、該当する演算子トレイト（または `#[derive(PartialEq)]`）を実装してください。";

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
            "演算子が定義されていない型に対して二項演算（`+`, `-`, `*` 等）を適用した場合に発生します。",
            "演算子トレイト（`Add`, `Sub` 等）の未実装が原因です。",
            "適切なメソッドを使用するか、演算子トレイトを実装してください。",
        )
    }
}
