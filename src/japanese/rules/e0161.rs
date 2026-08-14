use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0161;

impl DiagnosticRule for E0161 {
    fn code(&self) -> &'static str {
        "E0161"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "サイズが確定していない型（`!Sized` / `dyn Trait` / `[T]`）の値を直接ムーブできません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "トレイトオブジェクト（`dyn Trait`）やスライス（`[T]`）のように、コンパイル時にスタックサイズが不明な型の値を直接値渡し（ムーブ）しようとしました。";
        let reason = "Rust ではスタックに変数を配置するために `Sized`（サイズが確定していること）が前提となります。サイズ不定な値（Unsized Types）を直接ムーブすることはできません。";
        let solution = "`Box<dyn Trait>` や `&dyn Trait` などのポインタ・参照経由で扱ってください。";

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
            "サイズ未確定型（`dyn Trait` や `[T]` 等）を直接ムーブしようとした場合に発生します。",
            "スタックサイズの未定が原因です。",
            "`Box<dyn Trait>` などのポインタで間接化してください。",
        )
    }
}
