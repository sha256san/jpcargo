use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0594;

impl DiagnosticRule for E0594 {
    fn code(&self) -> &'static str {
        "E0594"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Mutability
    }

    fn title(&self) -> &'static str {
        "不変な参照先または変更不可な値の変更を試みています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "変更しようとしている値や参照先（デリファレンス先）が不変（イミュータブル）として宣言されているため、代入できません。";
        let reason = "Rust では不変参照（`&T`）や不変変数経由で中身のデータを書き換えることは固く禁じられています。";
        let solution = "1. 可変参照（`&mut T`）を使用してください。\n2. 変数定義に `mut` を追加してください。";

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
            "不変参照（`&T`）経由で参照先の値を変更しようとした場合に発生します。",
            "変更には可変参照（`&mut T`）が必要です。",
            "`&mut` を使用するか、内部可変性（`RefCell`, `Mutex` 等）を検討してください。",
        )
    }
}
