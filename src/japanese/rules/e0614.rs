use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0614;

impl DiagnosticRule for E0614 {
    fn code(&self) -> &'static str {
        "E0614"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "ポインタ・参照ではない型をデリファレンス（*演算子）しようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let type_re = Regex::new(r"type `(?P<type>[^`]+)` cannot be dereferenced").unwrap();
        let type_name = type_re
            .captures(&diag.message)
            .and_then(|c| c.name("type"))
            .map(|m| m.as_str())
            .unwrap_or("該当の型");

        let summary = format!(
            "型「{}」は参照（`&`）やスマートポインタではないため、`*`（デリファレンス演算子）を適用できません。",
            type_name
        );

        let reason = "`*` 演算子は参照（`&T`, `&mut T`）や `Deref` トレイトを実装した型（`Box<T>`, `Rc<T>` 等）にのみ使用できます。プリミティブ値（`i32` など）には適用できません。";
        let solution = "不要な `*` 演算子を削除してください。";

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
            "参照やポインタ以外の通常の値に対して `*`（参照外し）を行った場合に発生します。",
            "`Deref` トレイトが実装されていない型への不正なデリファレンスが原因です。",
            "`*` 演算子を取り除いてください。",
        )
    }
}
