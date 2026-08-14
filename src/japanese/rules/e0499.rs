use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0499;

impl DiagnosticRule for E0499 {
    fn code(&self) -> &'static str {
        "E0499"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "同時に複数の可変借用（&mut）が存在しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot borrow `(?P<var>[^`]+)` as mutable more than once at a time").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の変数");

        let summary = format!(
            "変数「{}」はすでに一度可変借用（&mut）されています。その借用が有効な間に、再度可変借用することはできません。",
            var_name
        );

        let reason = "Rust の借用規則の最重要原則：\n\
            「あるデータへの可変参照（`&mut`）は、同時に1つしか存在できない（排他制御）」\n\
            複数のポインタから同時にデータを書き換えるとデータ競合やイテレータの無効化（iterator invalidation）が発生するため、厳格に禁止されています。";

        let solution = "1. 最初の可変参照の利用が終わった後に、次の可変参照を作成してください。\n\
            2. ブロック `{ ... }` を使って最初の可変参照のスコープを制限してください。\n\
            3. 構造体の別々のフィールドを借用するように設計を見直してください。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("「同時に書き込める人は常に1人だけ」というRustのルールです。前の `&mut` の作業が終わるまで待ちましょう。".to_string());
        jd.expert_note = Some("Aliasing XOR Mutability 原則に違反。LLVM の noalias 最適化の前提条件を保証するためにコンパイラが拒絶します。".to_string());
        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "同一の変数に対して、重複して可変参照（&mut）を取得しようとした場合に発生します。",
            "データ競合を防ぐため、可変参照はスコープ内で唯一（排他的）である必要があります。",
            "参照の利用順序を分けるか、スコープをブロックで分離してください。",
        );
        jd.beginner_tip = Some("「同時に書き込める人は常に1人だけ」というRustのルールです。前の `&mut` の作業が終わるまで待ちましょう。".to_string());
        jd
    }
}
