use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0252;

impl DiagnosticRule for E0252 {
    fn code(&self) -> &'static str {
        "E0252"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "同じ名前が複数回インポート（use）されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let name_re = Regex::new(r"the name `(?P<name>[^`]+)` is defined multiple times").unwrap();
        let dup_name = name_re
            .captures(&diag.message)
            .and_then(|c| c.name("name"))
            .map(|m| m.as_str())
            .unwrap_or("該当の識別子");

        let summary = format!(
            "名前「{}」が複数の異なるモジュールからインポートされ、名前が衝突しています。",
            dup_name
        );

        let reason = "異なる場所から同名の項目をそのまま `use` すると、どちらの項目を指すか曖昧になります。";
        let solution = format!("`as` キーワードを使って別名（エイリアス）をつけてください。\n例: `use module_b::{} as Other{};`", dup_name, dup_name);

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
            "異なるモジュールから同一名称の項目がインポートされ衝突した場合に発生します。",
            "インポート名の重複が原因です。",
            "`use ... as Alias;` で別名を定義して衝突を回避してください。",
        )
    }
}
