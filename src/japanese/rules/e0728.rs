use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0728;

impl DiagnosticRule for E0728 {
    fn code(&self) -> &'static str {
        "E0728"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Async
    }

    fn title(&self) -> &'static str {
        "`await` は `async` 関数または `async` ブロックの内部でのみ使用できます"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "通常の同期関数の中で `.await` を呼び出そうとしました。";
        let reason = "`.await` は非同期タスクの完了を待機して制御をイベントループ/ランタイムに戻すキーワードであるため、`async fn` や `async { ... }` のコンテキスト内でのみ実行可能です。";
        let solution = "1. 関数を `async fn` に変更してください（例: `async fn main()`）。\n2. `main` 関数で非同期処理を行う場合は、Tokio などの非同期ランタイム（`#[tokio::main]`）を使用してください。\n3. ブロック内で `async { ... }` を使用してください。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("非同期関数を呼び出すときは、呼び出し元の関数にも `async fn` を付け、`#[tokio::main]` などのランタイムを導入しましょう。".to_string());
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
            "非同期（async）コンテキスト外で `.await` を実行した場合に発生します。",
            "`.await` は `async` スコープ内でのみ動作します。",
            "関数を `async fn` にするか、非同期ランタイム属性（`#[tokio::main]` 等）を付与してください。",
        )
    }
}
