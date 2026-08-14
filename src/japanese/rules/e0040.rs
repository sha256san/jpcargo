use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0040;

impl DiagnosticRule for E0040 {
    fn code(&self) -> &'static str {
        "E0040"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Memory
    }

    fn title(&self) -> &'static str {
        "`Drop::drop` メソッドを直接呼び出すことは禁止されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "値のデストラクタメソッド `x.drop()` を手動で明示的に呼び出そうとしました。";
        let reason = "`x.drop()` を手動で呼んでしまうと、変数がスコープを抜けた際に自動的にもう一度デストラクタが実行され、二重解放（double free）の未定義動作が生じるため、Rust は明示的な `.drop()` 呼び出しを構文レベルで禁止しています。";
        let solution = "明示的に値を早期破棄したい場合は、標準ライブラリの `drop(x)` 関数を使用してください（例: `std::mem::drop(x);`）。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("「`x.drop()`」ではなく「`drop(x)`」を使いましょう。`drop(x)` は所有権を奪ってスコープを終了させる安全な関数です。".to_string());
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
            "`x.drop()` を直接呼び出した場合に発生します。",
            "二重解放（double free）を防ぐための言語制約です。",
            "明示的な破棄には `std::mem::drop(x)` を使用してください。",
        )
    }
}
