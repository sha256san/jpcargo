use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0428;

impl DiagnosticRule for E0428 {
    fn code(&self) -> &'static str {
        "E0428"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "同一スコープ内で同じ名前が重複して定義されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let name_re = Regex::new(r"a (value|type|module|struct|function) named `(?P<name>[^`]+)` has already been defined").unwrap();
        let dup_name = name_re
            .captures(&diag.message)
            .and_then(|c| c.name("name"))
            .map(|m| m.as_str())
            .unwrap_or("該当の識別子");

        let summary = format!(
            "同じモジュールまたはスコープ内で、名前「{}」が2回以上定義されています。",
            dup_name
        );

        let reason = "Rust では同じ名前空間（ネームスペース）内で同名の項目を重複定義することはできません。";
        let solution = format!("どちらか一方の名前を変更するか、モジュールを分けてスコープを分離してください。");

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
            "同一スコープ内で同名の関数・構造体・変数が複数回定義された場合に発生します。",
            "名前の衝突が原因です。",
            "識別子名を変更するか、サブモジュールに分割してください。",
        )
    }
}
