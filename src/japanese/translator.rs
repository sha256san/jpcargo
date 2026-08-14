use std::collections::HashMap;
use crate::diagnostic::{classify, format_location, format_snippet, Diagnostic};
use crate::japanese::rules::{all_rules, DiagnosticRule};
use crate::japanese::template::JapaneseDiagnostic;

pub struct Translator {
    rules: HashMap<&'static str, Box<dyn DiagnosticRule>>,
}

impl Translator {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        for rule in all_rules() {
            rules.insert(rule.code(), rule);
        }
        Self { rules }
    }

    pub fn translate(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        if let Some(code_obj) = &diag.code {
            if let Some(rule) = self.rules.get(code_obj.code.as_str()) {
                return rule.explain(diag);
            }
        }

        // フォールバック（未対応エラーの場合）
        self.fallback(diag)
    }

    pub fn explain_code(&self, code: &str) -> Option<JapaneseDiagnostic> {
        let normalized = code.to_uppercase();
        self.rules.get(normalized.as_str()).map(|r| r.general_explanation())
    }

    fn fallback(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let code = diag
            .code
            .as_ref()
            .map(|c| c.code.clone())
            .unwrap_or_else(|| "UNKNOWN".to_string());

        let category = classify(&code);
        let title = if diag.level == "warning" {
            "コンパイラ警告 (Warning)"
        } else {
            "コンパイラエラー (Error)"
        };

        let summary = format!(
            "エラーメッセージ: {}\n※ このエラーコード（{}）の詳細な日本語解説はまだ登録されていません。",
            diag.message, code
        );

        let reason = "rustc コンパイラが出力した診断情報（原文）をそのまま表示しています。";
        let solution = "上記のエラーメッセージおよびコンパイラのヒント（help/note）を参照してコードを修正してください。";

        let mut jd = JapaneseDiagnostic::new(
            code,
            category,
            &diag.level,
            title,
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
}
