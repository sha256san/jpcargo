use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct DeadCode;

impl DiagnosticRule for DeadCode {
    fn code(&self) -> &'static str {
        "dead_code"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lint
    }

    fn title(&self) -> &'static str {
        "定義された関数・構造体・定数がどこからも呼び出されていません（デッドコード）"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let item_re = Regex::new(r"(function|struct|enum|constant|field|method) `(?P<item>[^`]+)` is never (used|read|constructed)").unwrap();
        let item_name = item_re
            .captures(&diag.message)
            .and_then(|c| c.name("item"))
            .map(|m| m.as_str())
            .unwrap_or("該当のコード要素");

        let summary = format!(
            "「{}」が定義されていますが、プログラム内のどこからも呼び出されたり参照されていません。",
            item_name
        );

        let reason = "バイナリサイズ肥大化やメンテナンス性の悪化を防ぐため、到達不能・未使用なコード要素を検出しています。";
        let solution = format!(
            "1. 使われていないコードであれば削除してください。\n\
            2. ライブラリクレートとして外部に公開する場合は `pub` を付与してください。\n\
            3. 意図的に残す場合は `#[allow(dead_code)]` を付与してください。"
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
            "warning",
            self.title(),
            "どこからも使用されていない関数・構造体・定数・メソッドが存在する場合に発生します。",
            "コードの肥大化・未整理を防ぐための検査です。",
            "不要なコードを削除するか、公開APIであれば `pub` を付与してください。",
        )
    }
}
