use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0119;

impl DiagnosticRule for E0119 {
    fn code(&self) -> &'static str {
        "E0119"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "同じ Trait が同じ型に対して重複して実装されています (Conflicting implementations)"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "同じ型に対して、同じ Trait の `impl` ブロックが2つ以上存在するか、ブランケット実装と競合しています。";
        let reason = "Rust では型と Trait の組み合わせに対して実装は唯一でなければなりません。重複があるとコンパイラがどちらの実装を呼び出すべきか判断できません。";
        let solution = "重複している `impl` ブロックのどちらか一方を削除するか、型または Trait を分離してください。";

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
            "同一の型に対する同一Traitの実装が競合・重複している場合に発生します。",
            "Trait実装の一意性（コヒーレンス規則）に違反しています。",
            "重複している実装を削除するか、ニュータイプパターン（`struct MyType(Type);`）で包んでください。",
        )
    }
}
