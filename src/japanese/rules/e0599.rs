use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0599;

impl DiagnosticRule for E0599 {
    fn code(&self) -> &'static str {
        "E0599"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "指定された型に対応するメソッドが見つかりません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let method_re = Regex::new(r"no method named `(?P<method>[^`]+)` found for (type|struct|enum) `(?P<type>[^`]+)`").unwrap();
        let (method_name, type_name) = if let Some(caps) = method_re.captures(&diag.message) {
            (
                caps.name("method").map(|m| m.as_str()).unwrap_or("メソッド"),
                caps.name("type").map(|m| m.as_str()).unwrap_or("型"),
            )
        } else {
            ("指定されたメソッド", "指定された型")
        };

        let summary = format!(
            "型「{}」には、メソッド「{}」が定義されていないか、スコープ内で利用可能になっていません。",
            type_name, method_name
        );

        let reason = "考えられる原因：\n\
            1. メソッド名のスペルミス（タイポ）\n\
            2. そのメソッドを提供する **Trait が現在のスコープに `use` でインポートされていない**\n\
            3. 引数の型やレシーバ（`&self` か `&mut self` か）が一致していない";

        let solution = format!(
            "1. メソッド名「{}」の綴りを確認してください。\n\
            2. Traitのメソッドである場合は、該当するTraitを `use` でインポートしてください（例: `use std::io::Read;` 等）。\n\
            3. 可変性が必要なメソッドの場合は、変数を `mut` にしてください。",
            method_name
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

        jd.beginner_tip = Some("Rustでは、Traitで定義されたメソッドを使うには、そのTrait自体を `use` してスコープに呼び出す必要があります。".to_string());
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
            "呼び出そうとしたメソッドが型に存在しない、またはTraitがインポートされていない場合に発生します。",
            "メソッドの定義漏れ、タイポ、またはTraitのスコープ外が主な原因です。",
            "メソッド名を確認するか、必要なTraitを `use` でインポートしてください。",
        )
    }
}
