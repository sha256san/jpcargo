use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
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
            .unwrap_or("未処理のパターン");

        let summary = format!(
            "`match` 式ですべての可能性が処理されていません。パターン「{}」の処理が不足しています。",
            missing_pat
        );

        let reason = "Rust の `match` は完全網羅（exhaustive）が必須です。\n\
            すべての Enum バリアントや値の可能性を漏れなく分岐処理することで、実行時の予期せぬ未処理エラーを防止します。";

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
            format!("方法1: 不足しているパターン `{}` のアームを追加する", missing_pat),
            "match val { ... }",
            format!("match val {{\n      {} => {{ /* 処理 */ }},\n      ... \n  }}", missing_pat),
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: ワイルドカード `_ => ...` を追加してその他のケースを処理する",
            "match val { ... }",
            "match val {\n      ... \n      _ => { /* その他の処理 */ },\n  }",
        ));

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "`match` 式で取り得るすべての値・バリアントが網羅されていない場合に発生します。",
            "未処理のケースによる実行時クラッシュを防ぐため、完全な網羅性が求められます。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: 不足しているバリアントのアームを追加する",
            "match x { Terminator::TalkToMyHand => {} }",
            "match x { Terminator::TalkToMyHand => {}, Terminator::HastaLaVistaBaby => {} }",
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: ワイルドカード `_ =>` を追加する",
            "match x { Terminator::TalkToMyHand => {} }",
            "match x { Terminator::TalkToMyHand => {}, _ => {} }",
        ));
        jd
    }
}
