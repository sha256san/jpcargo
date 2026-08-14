use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0502;

impl DiagnosticRule for E0502 {
    fn code(&self) -> &'static str {
        "E0502"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "可変借用（&mut）と不変借用（&）が同時に存在しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot borrow `(?P<var>[^`]+)` as mutable because it is also borrowed as immutable").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の変数");

        let summary = format!(
            "変数「{}」はすでに不変借用（参照）されているため、同時に可変借用（&mut）することはできません。",
            var_name
        );

        let reason = "Rust の借用規則（Aliasing XOR Mutability）により、以下のいずれか一方しか許可されません:\n\
            - 1つ以上の不変参照（`&T`）が存在する\n\
            - 唯一の可変参照（`&mut T`）が存在する\n\
            参照経由で読み取っている最中にデータが書き換えられることによるデータ競合やメモリ破壊を防ぐためです。";

        let solution = "1. 不変参照の利用がすべて終わった後に可変借用を行うように順序を調整するか、\n\
            2. ブロック `{ ... }` で囲んで不変参照のスコープ（ライフタイム）を早期に終了させてください。".to_string();

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
            "不変参照と可変参照が同じスコープで同時に存在しようとした場合に発生します。",
            "データ競合を防ぐため、Rust では「複数の読み取り参照」か「単一の書き込み参照」のどちらかしか許可されません。",
            "参照のスコープを分離するか、利用順序を見直してください。",
        )
    }
}
