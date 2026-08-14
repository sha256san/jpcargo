use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0210;

impl DiagnosticRule for E0210 {
    fn code(&self) -> &'static str {
        "E0210"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "ジェネリックな Trait 実装において、型パラメータの位置が孤児規則に違反しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "外部 Trait の実装で型パラメータ `T` を使用する際、ローカルな型よりも前に `T` が現れるような実装は禁止されています。";
        let reason = "将来的に外部クレート側で `impl<T> ForeignTrait for T` のようなブランケット実装が追加された際に衝突する可能性があるためです。";
        let solution = "ニュータイプパターンを使用するか、型パラメータの配置順序を見直してください。";

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
            "外部Traitに対するジェネリック実装において、孤児規則（コヒーレンス）に違反した場合に発生します。",
            "将来の外部クレート実装との衝突を防ぐための規則です。",
            "自作のラッパー構造体にTraitを実装してください。",
        )
    }
}
