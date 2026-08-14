use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0616;

impl DiagnosticRule for E0616 {
    fn code(&self) -> &'static str {
        "E0616"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Visibility
    }

    fn title(&self) -> &'static str {
        "構造体の非公開（private）フィールドに直接アクセスしようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let field_re = Regex::new(r"field `(?P<field>[^`]+)` of struct `(?P<struct>[^`]+)` is private").unwrap();
        let (field_name, struct_name) = if let Some(caps) = field_re.captures(&diag.message) {
            (
                caps.name("field").map(|m| m.as_str()).unwrap_or("フィールド"),
                caps.name("struct").map(|m| m.as_str()).unwrap_or("構造体"),
            )
        } else {
            ("指定のフィールド", "指定の構造体")
        };

        let summary = format!(
            "構造体「{}」のフィールド「{}」は非公開（private）であるため、モジュール外から直接読み書きできません。",
            struct_name, field_name
        );

        let reason = "Rust の構造体フィールドはデフォルトで `private` です。\n\
            外部モジュールからアクセスできるようにするには、フィールド定義に `pub` を付けるか、公開ゲッター/セッターメソッドを経由する必要があります。";

        let solution = format!(
            "1. フィールド定義に `pub` を付けて公開してください（例: `pub {}: ...`）。\n\
            2. 公開されているゲッターメソッド（例: `.{}()`）を使用してください。",
            field_name, field_name
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
            "非公開構造体フィールドへモジュール外部から直接アクセスした場合に発生します。",
            "カプセル化（情報隠蔽）ルールによる制約です。",
            "フィールドに `pub` を付与するか、アクセサメソッドを提供してください。",
        )
    }
}
