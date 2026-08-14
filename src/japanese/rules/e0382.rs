use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0382;

impl DiagnosticRule for E0382 {
    fn code(&self) -> &'static str {
        "E0382"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "所有権が移動（ムーブ）した値を再び使用しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"use of moved value: `(?P<var>[^`]+)`").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の値");

        let summary = format!(
            "変数「{}」の所有権はすでに別の場所へ移動（ムーブ）しているため、ここで再利用することはできません。",
            var_name
        );

        let reason = "Rust では、`Copy` トレイトを実装していない型（String や Vec 等）を別の変数に代入したり関数に渡すと、\n\
            所有権が移動（ムーブ）し、元の変数は無効化されます。\n\
            これにより、二重解放（double free）などの重大なメモリバグを防いでいます。";

        let solution = format!(
            "以下のいずれかの方法で解決してください:\n\
            1. 所有権を渡す代わりに **参照（借用）** を渡す（例: `&{}`）\n\
            2. データを複製する（例: `{}.clone()`）\n\
            3. 値の利用順序を見直す",
            var_name, var_name
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

        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            if child.level == "note" || child.level == "help" {
                jd.suggestions.push(format!("{}: {}", child.level, child.message));
            }
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "所有権がすでにムーブされた変数を後続のコードで読み取ろうとすると発生します。",
            "Rust のメモリ管理原則では、各値は単一の所有者を持ち、ムーブ後は元の変数が無効になります。",
            "参照（`&`）を渡すか、`.clone()` で複製を作成してください。",
        )
    }
}
