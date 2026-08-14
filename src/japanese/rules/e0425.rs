use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0425;

impl DiagnosticRule for E0425 {
    fn code(&self) -> &'static str {
        "E0425"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::NameResolution
    }

    fn title(&self) -> &'static str {
        "このスコープ内に指定された値・変数が見つかりません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"cannot find value `(?P<var>[^`]+)` in this scope").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の識別子");

        let summary = format!(
            "識別子「{}」は、現在のスコープで定義されていないか、インポートされていません。",
            var_name
        );

        let reason = "Rust では、使用するすべての変数・関数・定数は、事前に宣言されているか、\n\
            `use` 宣言によって現在のモジュールスコープにインポートされている必要があります。";

        let solution = format!(
            "1. 「{}」のスペルミスがないか確認してください。\n\
            2. `let {} = ...;` で変数を定義するか、\n\
            3. 外部クレートやモジュールから `use ...::{};` でインポートしてください。",
            var_name, var_name, var_name
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
            "参照しようとした変数名・定数名・関数名が未定義の場合に発生します。",
            "スコープ外か、インポート漏れ、またはタイポ（綴り間違い）が原因です。",
            "スペルを確認するか、`use` でインポート、あるいは変数を定義してください。",
        )
    }
}
