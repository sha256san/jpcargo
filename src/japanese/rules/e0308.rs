use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0308;

impl DiagnosticRule for E0308 {
    fn code(&self) -> &'static str {
        "E0308"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "型が一致していません (Mismatched Types)"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let type_re = Regex::new(r"expected `(?P<exp>[^`]+)`, found `(?P<found>[^`]+)`").unwrap();
        let (expected, found) = if let Some(caps) = type_re.captures(&diag.message) {
            (
                caps.name("exp").map(|m| m.as_str()).unwrap_or("不明"),
                caps.name("found").map(|m| m.as_str()).unwrap_or("不明"),
            )
        } else {
            ("期待された型", "指定された型")
        };

        let summary = format!(
            "期待されている型は `{}` ですが、実際に渡された値の型は `{}` です。",
            expected, found
        );

        let reason = format!(
            "Rust は強い静的型付け言語です。\n\
            関数の戻り値、変数、関数の引数などですべての型が厳密に一致している必要があります。\n\
            暗黙的な型変換は自動で行われません（明示的な変換や参照が必要です）。"
        );

        let solution = format!(
            "1. 渡す値の型を `{}` に変換するか（例: `.to_string()`, `as i32`, `&` など）、\n\
            2. 受け取り側（変数や戻り値の型シグネチャ）を `{}` に変更してください。",
            expected, found
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
            if child.level == "help" {
                jd.suggestions.push(format!("ヒント: {}", child.message));
            } else if child.level == "note" {
                jd.suggestions.push(format!("補足: {}", child.message));
            }
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "式や引数の型が、コンパイラが要求する型と一致していない場合に発生します。",
            "Rust では暗黙の型キャストが行われないため、型を完全に一致させる必要があります。",
            "適切な型変換メソッド（`.parse()`, `.into()`, `as`, `&` 等）を使用するか、型シグネチャを修正してください。",
        )
    }
}
