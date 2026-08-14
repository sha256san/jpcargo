use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0603;

impl DiagnosticRule for E0603 {
    fn code(&self) -> &'static str {
        "E0603"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Visibility
    }

    fn title(&self) -> &'static str {
        "非公開（private）な項目にモジュール外からアクセスしようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let item_re = Regex::new(r"(struct|function|module|constant|type alias|trait) `(?P<item>[^`]+)` is private").unwrap();
        let item_name = item_re
            .captures(&diag.message)
            .and_then(|c| c.name("item"))
            .map(|m| m.as_str())
            .unwrap_or("該当の項目");

        let summary = format!(
            "項目「{}」はモジュール内でプライベート（非公開）として宣言されているため、外部からアクセスできません。",
            item_name
        );

        let reason = "Rust では、すべての項目（モジュール、構造体、関数、トレイト等）はデフォルトで `private` です。\n\
            外部のモジュールからアクセスできるようにするには、明示的に `pub` キーワードを付ける必要があります。";

        let solution = format!(
            "1. 該当項目の定義元で `pub`（または `pub(crate)`）キーワードを追加して公開してください。\n\
            例: `pub fn {}() {{ ... }}` または `pub struct {} {{ ... }}`\n\
            2. 公開すべきでない内部実装の場合は、提供されている公開 API を経由して利用してください。",
            item_name, item_name
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
            "非公開（private）な関数や構造体に、モジュールの外側からアクセスしようとした場合に発生します。",
            "Rust のカプセル化機能により、`pub` が付いていない項目はモジュール外から隠蔽されます。",
            "定義元に `pub` を付けるか、公開されている代替APIを使用してください。",
        )
    }
}
