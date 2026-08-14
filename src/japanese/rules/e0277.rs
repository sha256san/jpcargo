use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0277;

impl DiagnosticRule for E0277 {
    fn code(&self) -> &'static str {
        "E0277"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "型が必要なトレイトを実装していません (Trait Bound Not Satisfied)"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let trait_re = Regex::new(r"the trait bound `(?P<bound>[^`]+)` is not satisfied").unwrap();
        let bound_str = trait_re
            .captures(&diag.message)
            .and_then(|c| c.name("bound"))
            .map(|m| m.as_str())
            .unwrap_or("要求されたトレイト");

        let summary = format!(
            "指定された型は、要求されているトレイト境界 `{}` を満たしていません。",
            bound_str
        );

        let reason = "Rust のジェネリック関数や構造体、標準マクロ（`println!(\"{:?}\", ...)` 等）は、\n\
            引数や型に対して特定の振る舞い（トレイト: `Debug`, `Display`, `Clone` など）を要求します。\n\
            そのトレイトが実装されていない型を渡したため、コンパイルエラーが発生しました。";

        let solution = format!(
            "1. 独自定義の構造体や Enum であれば、`#[derive(...)]` でトレイトを自動導出してください。\n\
            2. 手動で `impl ... for Type {{ ... }}` を実装してください。"
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
            "要求されたトレイト（インターフェース仕様）を型が実装していない場合に発生します。",
            "ジェネリクス境界やマクロの要件を満たす必要があります。",
            "`#[derive(...)]` を追加するか、手動で `impl` を記述してください。",
        )
    }
}
