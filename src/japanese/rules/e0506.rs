use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0506;

impl DiagnosticRule for E0506 {
    fn code(&self) -> &'static str {
        "E0506"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "借用中の変数に値を代入・再バインドできません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot assign to `(?P<var>[^`]+)` because it is borrowed").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の変数");

        let summary = format!(
            "変数「{}」は現在他の場所で借用（参照）されているため、新しい値を代入できません。",
            var_name
        );

        let reason = "他の変数や関数がこの変数を参照している最中に値が変更されると、\n\
            参照先の内容が予期せず変わったり、不正なポインタが生じる危険があるため Rust コンパイラが禁止しています。";

        let solution = "すべての借用（参照）の利用が完了した後に値を代入するようにコードを修正してください。";

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
            "借用中の変数に対して代入や上書きを行おうとした場合に発生します。",
            "既存の参照が存在する状態で元のデータを書き換えることは安全上禁止されています。",
            "借用が終了してから代入を行ってください。",
        )
    }
}
