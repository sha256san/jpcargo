use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0433;

impl DiagnosticRule for E0433 {
    fn code(&self) -> &'static str {
        "E0433"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::NameResolution
    }

    fn title(&self) -> &'static str {
        "未宣言のクレートまたはモジュールが使用されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let mod_re = Regex::new(r"failed to resolve: use of undeclared (crate or )?module `(?P<mod>[^`]+)`").unwrap();
        let mod_name = mod_re
            .captures(&diag.message)
            .and_then(|c| c.name("mod"))
            .map(|m| m.as_str())
            .unwrap_or("該当のモジュール");

        let summary = format!(
            "モジュールまたはクレート「{}」が見つかりません。",
            mod_name
        );

        let reason = "現在のプロジェクトで宣言されていないモジュールパス、または `Cargo.toml` に追加されていない外部クレートを参照しようとしています。";

        let solution = format!(
            "1. 外部クレートの場合は、`Cargo.toml` の `[dependencies]` に `{}` を追加してください。\n\
            2. 自作モジュールの場合は、`mod {};` がルートファイル（main.rs / lib.rs）に記述されているか確認してください。",
            mod_name, mod_name
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
            "存在しない、または未登録のモジュール/クレートを指定した場合に発生します。",
            "Cargo.toml への依存追加漏れや、`mod` 宣言の欠落が原因です。",
            "Cargo.toml やモジュール構造を見直してください。",
        )
    }
}
