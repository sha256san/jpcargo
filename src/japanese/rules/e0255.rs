use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0255;

impl DiagnosticRule for E0255 {
    fn code(&self) -> &'static str {
        "E0255"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "インポートした名前が、既存の定義（関数・構造体・変数等）と衝突しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "`use` でインポートしようとした名前が、同じモジュール内で定義された別の項目と重複しています。";
        let reason = "同名定義による名前の衝突（シャドーイングの不正）を防ぐためです。";
        let solution = "`use ... as OtherName;` でインポート側に別名をつけるか、定義名を変更してください。";

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
            "`use` インポートした名称がローカル定義と重複した場合に発生します。",
            "名前空間内での衝突が原因です。",
            "`as` でインポート名を変更してください。",
        )
    }
}
