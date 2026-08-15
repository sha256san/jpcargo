use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
use super::DiagnosticRule;

pub struct E0384;

impl DiagnosticRule for E0384 {
    fn code(&self) -> &'static str {
        "E0384"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Mutability
    }

    fn title(&self) -> &'static str {
        "不変（イミュータブル）変数に値を再代入しようとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot assign twice to immutable variable `(?P<var>[^`]+)`").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("a");

        let summary = format!(
            "変数「{}」は不変（immutable）として宣言されているため、2回目の値の代入（再代入）はできません。",
            var_name
        );

        let reason = "Rust では、`let` で宣言された変数はデフォルトでイミュータブル（不変）です。\n\
            初期値が設定された後に値を上書き・変更することは許可されていません。\n\
            変数を後から書き換える必要がある場合は、明示的に可変（`mut`）にする必要があります。";

        let solution = "";

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

        // 複数の修正方法とコード例（日本語コメント付き）
        jd.add_fix_option(FixOption::diff(
            "方法1: 変数宣言に `mut` を追加して可変変数にする（推奨）",
            format!("let {} = ...;", var_name),
            format!("let mut {} = ...;", var_name),
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: `let` を付けて新しい変数として再定義（シャドーイング）する",
            format!("{} = ...;", var_name),
            format!("let {} = ...;", var_name),
        ));

        for child in &diag.children {
            if child.level == "help" {
                jd.suggestions.push(format!("ヒント: {}", child.message));
            } else if child.level == "note" {
                jd.suggestions.push(format!("補足: {}", child.message));
            }
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "すでに初期化された不変変数に対して、2回目の値の代入を行おうとすると発生します。",
            "Rust の変数はデフォルトで不変（再代入不可）です。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: 変数宣言に `mut` を追加する",
            "let x = ...;",
            "let mut x = ...;",
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: let でシャドーイングする",
            "x = ...;",
            "let x = ...;",
        ));
        jd
    }
}
