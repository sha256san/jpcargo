use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
use super::DiagnosticRule;

pub struct E0072;

impl DiagnosticRule for E0072 {
    fn code(&self) -> &'static str {
        "E0072"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Struct
    }

    fn title(&self) -> &'static str {
        "構造体が直接自身を含んでいるため、型のサイズが無限大になります"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let type_re = Regex::new(r"recursive type `(?P<type>[^`]+)` has infinite size").unwrap();
        let type_name = type_re
            .captures(&diag.message)
            .and_then(|c| c.name("type"))
            .map(|m| m.as_str())
            .unwrap_or("Node");

        let summary = format!(
            "構造体「{}」が自身と同じ型を直接フィールドに持っているため、メモリサイズをコンパイル時に計算できません。",
            type_name
        );

        let reason = "Rust の構造体はスタック上に固定サイズで確保されます。\n\
            構造体の中に自分自身を直接埋め込むと、入れ子構造が無限に続くことになり（マトリョーシカ状態）、無限のメモリサイズが必要になってしまいます。";

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
            format!("方法1: `Box<{}>` を使用してヒープ確保・間接参照にする（推奨）", type_name),
            format!("next: Option<{}>,", type_name),
            format!("next: Option<Box<{}>>,", type_name),
        ));
        jd.add_fix_option(FixOption::diff(
            format!("方法2: 複数所有が必要な場合は `Rc<{}>` または `Arc<{}>` を使用する", type_name, type_name),
            format!("next: Option<{}>,", type_name),
            format!("next: Option<std::rc::Rc<{}>>,", type_name),
        ));

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
            "再帰的なデータ構造（自己参照構造体）が直接自身を含んでいる場合に発生します。",
            "スタック上のサイズを静的に決定できないためです。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: `Box<T>` を使用してサイズを固定化する",
            "struct Node { next: Option<Node> }",
            "struct Node { next: Option<Box<Node>> }",
        ));
        jd
    }
}
