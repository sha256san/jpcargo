use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0606;

impl DiagnosticRule for E0606 {
    fn code(&self) -> &'static str {
        "E0606"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "不正な型キャスト（`as`）を行おうとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "`as` 演算子で直接キャストできない型同士の間で型変換を行おうとしました（例: `&str as i32` など）。";
        let reason = "`as` キーワードは基本プリミティブ数値型間の変換（`i32 as f64` 等）や生ポインタ変換などの直接キャストのみをサポートしています。文字列や複雑な型の変換には専用のパース・変換メソッドが必要です。";
        let solution = "1. 文字列から数値への変換: `\"123\".parse::<i32>()` を使用してください。\n2. トレイト変換: `From` / `Into` / `TryFrom` トレイトを使用してください。";

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
            "`as` 演算子で変換できない型同士のキャストを行った場合に発生します。",
            "`as` はプリミティブ数値等の制限された変換のみサポートします。",
            "`.parse()` や `.into()` などの適切な変換メソッドを使用してください。",
        )
    }
}
