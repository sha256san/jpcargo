use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0505;

impl DiagnosticRule for E0505 {
    fn code(&self) -> &'static str {
        "E0505"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "他の場所で借用（参照）されている値を移動（ムーブ）できません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot move out of `(?P<var>[^`]+)` because it is borrowed").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の変数");

        let summary = format!(
            "値「{}」は現在他の場所で借用（参照）されている最中のため、所有権を別の場所へ移動（ムーブ）することはできません。",
            var_name
        );

        let reason = "参照が存在している間に元の値をムーブしてしまうと、その参照は無効なメモリ領域を指すことになります（ダングリングポインタの発生）。\n\
            メモリ安全性を保証するため、すべての借用が終了するまで値の所有権移動は禁止されています。";

        let solution = format!(
            "1. 参照を使っている処理が終わった後にムーブを行うか、\n\
            2. ムーブする代わりに `.clone()` で複製を渡すか、\n\
            3. 所有権の代わりに参照（`&{}`）を渡してください。",
            var_name
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

        jd.beginner_tip = Some("「友達に本を貸している最中に、その本を他の人にあげたり捨てたりすることはできない」というイメージです。".to_string());
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
            "参照されているデータを、参照の有効期間中にムーブ（所有権移動）しようとすると発生します。",
            "既存の参照が無効なメモリ（解放済みメモリ）を指してしまうのを防ぐための規則です。",
            "参照のスコープが終了してからムーブするか、`.clone()` を使用してください。",
        )
    }
}
