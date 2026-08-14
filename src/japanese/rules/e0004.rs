use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0004;

impl DiagnosticRule for E0004 {
    fn code(&self) -> &'static str {
        "E0004"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "match 式のパターンが網羅されていません (Non-exhaustive patterns)"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let pattern_re = Regex::new(r"non-exhaustive patterns: `(?P<pat>[^`]+)` not covered").unwrap();
        let missing_pat = pattern_re
            .captures(&diag.message)
            .and_then(|c| c.name("pat"))
            .map(|m| m.as_str())
            .unwrap_or("一部のパターン");

        let summary = format!(
            "`match` 式ですべての可能性が処理されていません。パターン「{}」の処理が不足しています。",
            missing_pat
        );

        let reason = "Rust の `match` は完全網羅（exhaustive）が必須です。\n\
            すべての Enum バリアントや値の可能性を漏れなく分岐処理することで、実行時の予期せぬ未処理エラーを防止します。";

        let solution = format!(
            "1. 不足しているパターン `{}` のアームを追加してください。\n\
            2. または、ワイルドカード `_ => ...` を追加してその他のケースを処理してください。",
            missing_pat
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

        jd.beginner_tip = Some("Enumの選択肢が増えたときなど、処理漏れをコンパイラが未然に教えてくれるRustの強力な機能です。".to_string());
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
            "`match` 式で取り得るすべての値・バリアントが網羅されていない場合に発生します。",
            "未処理のケースによる実行時クラッシュを防ぐため、完全な網羅性が求められます。",
            "不足しているバリアントのアームを追加するか、`_ =>`（ワイルドカード）を追加してください。",
        )
    }
}
