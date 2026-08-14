use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct UnusedMustUse;

impl DiagnosticRule for UnusedMustUse {
    fn code(&self) -> &'static str {
        "unused_must_use"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lint
    }

    fn title(&self) -> &'static str {
        "`#[must_use]` が指定された重要な戻り値（`Result` 等）が無視・破棄されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let type_re = Regex::new(r"unused `(?P<t>[^`]+)` that must be used").unwrap();
        let type_name = type_re
            .captures(&diag.message)
            .and_then(|c| c.name("t"))
            .map(|m| m.as_str())
            .unwrap_or("Result / 返り値");

        let summary = format!(
            "関数の戻り値「{}」はエラーハンドリングや処理結果の確認が必須（`#[must_use]`）ですが、結果を受け取らずに無視されています。",
            type_name
        );

        let reason = "Rust では `Result` や `Option` を無視すると、ファイルI/Oの失敗やネットワークエラーに気づけず重大なバグにつながるため、強力に警告されます。";
        let solution = "1. `?` 演算子でエラーを伝播させるか、`match` や `if let` で結果をチェックしてください。\n\
            2. 意図的に結果を捨てる場合は `let _ = ...;` で明示的に受けてください。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("「成功したか失敗したかの結果を捨てています」。エラーが起きたときにクラッシュしないよう、`?` を付けるか `match` で結果を確認しましょう。".to_string());
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
            "warning",
            self.title(),
            "`Result` や `#[must_use]` な関数の返り値がハンドリングされずに破棄された場合に発生します。",
            "エラーの見落としやリソースリークを防ぐための設計です。",
            "`?` 演算子や `match` で結果を適切に処理してください。",
        )
    }
}
