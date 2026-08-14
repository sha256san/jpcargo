use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0207;

impl DiagnosticRule for E0207 {
    fn code(&self) -> &'static str {
        "E0207"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Generic
    }

    fn title(&self) -> &'static str {
        "impl ブロックの型パラメータが制約（使用）されていません (Unconstrained type parameter)"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "`impl<T> Struct { ... }` や `impl<T> Trait for Struct` のように宣言された型パラメータ `T` が、実装対象の型や Trait 内で一切使用されていません。";
        let reason = "使用されていない型パラメータが存在すると、型推論時に `T` を決定できず、曖昧さやコンパイル不能を引き起こします。";
        let solution = "1. `impl` から不要な型パラメータ `<T>` を削除するか、\n2. メソッド側のジェネリクス（例: `fn method<T>(...)`）に移動してください。";

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
            "`impl` で宣言された型パラメータが、実装対象の型やTraitに現れない場合に発生します。",
            "未束縛の型パラメータによる曖昧性を防ぐための規則です。",
            "`impl` レベルではなく個別のメソッドレベルで型パラメータを宣言してください。",
        )
    }
}
