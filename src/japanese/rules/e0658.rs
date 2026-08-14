use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0658;

impl DiagnosticRule for E0658 {
    fn code(&self) -> &'static str {
        "E0658"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Feature
    }

    fn title(&self) -> &'static str {
        "Nightly限定の不安定な機能（unstable feature）を使用しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let feature_re = Regex::new(r"use of unstable library feature '(?P<feat>[^']+)'|use of unstable library feature `(?P<feat2>[^`]+)`").unwrap();
        let feat_name = if let Some(caps) = feature_re.captures(&diag.message) {
            caps.name("feat").or_else(|| caps.name("feat2")).map(|m| m.as_str()).unwrap_or("該当の機能")
        } else {
            "該当の機能"
        };

        let summary = format!(
            "機能「{}」はまだ安定化（Stable化）されていない開発中の機能であるため、Stable Rust では使用できません。",
            feat_name
        );

        let reason = "Rust では実験的な新機能は Nightly チャンネル限定で提供され、安定版（Stable）コンパイラではデフォルトで無効化されています。";
        let solution = format!(
            "1. Nightly ツールチェーンに切り替えて、クレートルートに `#![feature({})]` を追加してください。\n\
            2. または、Stable で利用可能な安定版の代替APIを使用してください。",
            feat_name
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
            "Nightly限定の実験的機能（unstable feature）をStable版で呼び出した場合に発生します。",
            "言語仕様の安定性保護のための制約です。",
            "Nightlyコンパイラと `#![feature(...)]` を使用するか、安定版APIを利用してください。",
        )
    }
}
