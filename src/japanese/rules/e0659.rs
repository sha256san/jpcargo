use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0659;

impl DiagnosticRule for E0659 {
    fn code(&self) -> &'static str {
        "E0659"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Macro
    }

    fn title(&self) -> &'static str {
        "同名のマクロが複数インポートされており、どれを使用するか曖昧です"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let macro_re = Regex::new(r"`(?P<macro>[^`]+)` is ambiguous").unwrap();
        let macro_name = macro_re
            .captures(&diag.message)
            .and_then(|c| c.name("macro"))
            .map(|m| m.as_str())
            .unwrap_or("マクロ");

        let summary = format!(
            "マクロ「{}」の定義が複数のクレートやスコープに存在し、コンパイラが特定できません。",
            macro_name
        );

        let reason = "同名のマクロがスコープ内で競合しているため、完全修飾パスで指定する必要があります。";
        let solution = format!("マクロの完全修飾パス（例: `crate_name::{}!(...)`）で呼び出すか、不要な `use` を整理してください。", macro_name);

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
            "同一名称のマクロが複数スコープに存在し曖昧な場合に発生します。",
            "名前空間の競合が原因です。",
            "完全修飾パスでマクロを呼び出してください。",
        )
    }
}
