use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct NonSnakeCase;

impl DiagnosticRule for NonSnakeCase {
    fn code(&self) -> &'static str {
        "non_snake_case"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Style
    }

    fn title(&self) -> &'static str {
        "変数・関数・モジュール名が Rust 標準の命名規則（`snake_case`）に違反しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let name_re = Regex::new(r"should have a snake case name: `(?P<name>[^`]+)`").unwrap();
        let name_str = name_re
            .captures(&diag.message)
            .and_then(|c| c.name("name"))
            .map(|m| m.as_str())
            .unwrap_or("該当の名前");

        let summary = format!(
            "名前「{}」は大文字が含まれているなど、`snake_case`（小文字とアンダースコア）になっていません。",
            name_str
        );

        let reason = "Rust では関数、変数、モジュールの命名規則としてすべて小文字のスネークケース（例: `my_variable`, `calculate_sum`）が公式規約として定められています。";
        let solution = "すべて小文字とアンダースコアを使ったスネークケースにリネームしてください。";

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
            "warning",
            self.title(),
            "変数や関数の名前に大文字が使われているなど、`snake_case` に反する場合に発生します。",
            "コミュニティ全体のコード一貫性を保つための公式スタイル規則です。",
            "小文字とアンダースコアを用いた `snake_case` に修正してください。",
        )
    }
}
