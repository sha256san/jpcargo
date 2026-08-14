use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0080;

impl DiagnosticRule for E0080 {
    fn code(&self) -> &'static str {
        "E0080"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Const
    }

    fn title(&self) -> &'static str {
        "コンパイル時の定数評価（計算）に失敗しました"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "定数式（`const` や配列のサイズ指定など）のコンパイル時計算中に、ゼロ除算やオーバーフロー、不正なメモリアクセスが発生しました。";
        let reason = "コンパイラが定数式を評価した結果、数学的・メモリ的に不正な処理（例: `1 / 0`）が検出されました。";
        let solution = "定数式の計算内容（ゼロ除算、配列外参照、オーバーフロー等）を確認して修正してください。";

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
            "コンパイル時の定数計算（ゼロ除算・オーバーフロー等）が失敗した場合に発生します。",
            "定数評価中の異常が原因です。",
            "定数式の計算ロジックを見直してください。",
        )
    }
}
