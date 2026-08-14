use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct UnreachableCode;

impl DiagnosticRule for UnreachableCode {
    fn code(&self) -> &'static str {
        "unreachable_code"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lint
    }

    fn title(&self) -> &'static str {
        "絶対に実行されないコード（到達不能コード）が存在します"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "`return` や `panic!`、無限ループ（`loop`）の直後に文が書かれており、この行に制御が到達することはありません。";
        let reason = "プログラムの実行パスにおいて、先行する処理で関数が終了またはジャンプするため、以降のコードは決して実行されません。";
        let solution = "到達しない不要なコード行を削除するか、分岐条件を見直してください。";

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
            "warning",
            self.title(),
            "`return` や `panic!` の直後に不要なコードが存在する場合に発生します。",
            "デッドコードの排除とロジック整合性のための警告です。",
            "実行されない不要な文を削除してください。",
        )
    }
}
