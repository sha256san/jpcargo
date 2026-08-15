use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
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
                caps.name("exp").map(|m| m.as_str()).unwrap_or("期待された型"),
                caps.name("found").map(|m| m.as_str()).unwrap_or("指定された型"),
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

        let solution = "";

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

        // 複数の修正方法とコード例（日本語コメント付き）
        jd.add_fix_option(FixOption::diff(
            format!("方法1: 渡す側の値を期待される型 `{}` に変換する", expected),
            format!("let val: {} = ...;", expected),
            format!("let val: {} = 値.変換メソッド();", expected),
        ));
        jd.add_fix_option(FixOption::diff(
            format!("方法2: 受け取り側の型宣言を `{}` に合わせる", found),
            format!("let val: {} = ...;", expected),
            format!("let val: {} = ...;", found),
        ));

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
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "式や引数の型が、コンパイラが要求する型と一致していない場合に発生します。",
            "Rust では暗黙の型キャストが行われないため、型を完全に一致させる必要があります。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: 渡す側の値を変換する（例: .to_string(), .parse(), as 型 等）",
            "let num: u32 = \"42\";",
            "let num: u32 = \"42\".parse().unwrap();",
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: 変数側の型定義を変更する",
            "let num: u32 = \"42\";",
            "let num: &str = \"42\";",
        ));
        jd
    }
}
