use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0507;

impl DiagnosticRule for E0507 {
    fn code(&self) -> &'static str {
        "E0507"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "借用した参照先から値（所有権）を取り出すことはできません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot move out of `(?P<var>[^`]+)` which is behind a (?P<kind>[^`\n]+)").unwrap();
        let (var_name, ref_kind) = if let Some(caps) = var_re.captures(&diag.message) {
            (
                caps.name("var").map(|m| m.as_str()).unwrap_or("参照先"),
                caps.name("kind").map(|m| m.as_str()).unwrap_or("参照"),
            )
        } else {
            ("参照先", "参照")
        };

        let summary = format!(
            "「{}」は {} のため、所有権を持っていません。所有権を持たない参照先から値を取り出してムーブすることはできません。",
            var_name, ref_kind
        );

        let reason = "参照（`&T` や `&mut T`）はデータの「一時的な利用権」に過ぎず、「所有権」はありません。\n\
            もし参照先から値をムーブしてしまうと、元の所有者が持っているデータの中身が空っぽ（未初期化）になり、所有権の原則が破綻します。";

        let solution = "1. 所有権を奪うのではなく、参照のまま利用する（例: `&` を使う）\n\
            2. 対象の型が `Clone` を実装していれば `.clone()` で複製する\n\
            3. `Option::take()` や `std::mem::replace()` を使ってデフォルト値と入れ替える";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("「人から借りたものを勝手に誰かにプレゼントすることはできない」のと同じです。".to_string());
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
            "参照経由でアクセスしている値から、所有権をムーブしようとすると発生します。",
            "参照は借りているだけなので、中身を抜き取ることはできません。",
            "`.clone()` で複製するか、参照のまま操作してください。",
        )
    }
}
