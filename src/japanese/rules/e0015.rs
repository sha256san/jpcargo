use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0015;

impl DiagnosticRule for E0015 {
    fn code(&self) -> &'static str {
        "E0015"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Const
    }

    fn title(&self) -> &'static str {
        "const や static の定義で、定数コンテキスト外の関数呼び出しは許可されていません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "`const` や `static` 変数の初期化式で、`const fn` ではない通常の関数（ヒープ確保を行う関数など）を呼び出そうとしました。";
        let reason = "定数はコンパイル時に値が完全に計算・確定される必要があります。実行時にしか動かない一般的な関数や動的メモリ確保（`String::new()` 等）は直接使用できません。";
        let solution = "1. `const fn` を使用するか、文字列リテラル（`&'static str`）を使用してください。\n2. 実行時初期化が必要なグローバル変数の場合は、`std::sync::LazyLock`（または `once_cell` クレート）を使用してください。";

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
            "定数（`const`/`static`）の初期化で、非 `const fn` を呼び出した場合に発生します。",
            "コンパイル時定数評価の制約によるものです。",
            "`const fn` を使うか、`std::sync::LazyLock` を利用してください。",
        )
    }
}
