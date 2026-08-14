use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0117;

impl DiagnosticRule for E0117 {
    fn code(&self) -> &'static str {
        "E0117"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "孤児規則（Orphan Rule）違反：外部の型に対して外部の Trait を直接実装できません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "外部クレートで定義された型（例: `Vec<T>`, `i32` 等）に対して、同じく外部クレートで定義された Trait（例: `Display` 等）を実装しようとしています。";
        let reason = "Rust の孤児規則（Orphan Rule）では、「Trait または 型 の少なくとも一方が現在のクレートで定義されたローカルなもの」でなければ実装できません。\nこれにより、異なるクレート間で実装の衝突が起きるのを防ぎます。";
        let solution = "ニュータイプパターン（Newtype Pattern）を使用してください。\n例: `struct MyVec(Vec<i32>);` のように自分のクレートでタプル構造体を定義し、その構造体に Trait を実装します。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("「他人の型」に「他人のTrait」を勝手に実装することはできません。自分のラッパー構造体（Newtype）を作りましょう。".to_string());
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
            "外部クレートの型に外部クレートのTraitを実装しようとした場合に発生します（孤児規則違反）。",
            "クレート間での実装衝突を防ぐための言語制約です。",
            "タプル構造体でラップする「ニュータイプパターン」を使用してください。",
        )
    }
}
