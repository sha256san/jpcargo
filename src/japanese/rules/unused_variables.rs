use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct UnusedVariables;

impl DiagnosticRule for UnusedVariables {
    fn code(&self) -> &'static str {
        "unused_variables"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lint
    }

    fn title(&self) -> &'static str {
        "宣言された変数が一度も使用されていません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"unused variable: `(?P<var>[^`]+)`").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の変数");

        let summary = format!(
            "変数「{}」が定義されましたが、以降のコードで一度も読み取られていません。",
            var_name
        );

        let reason = "タイポやロジックの書き忘れを防ぐため、Rustコンパイラは未使用変数を警告として検知します。";
        let solution = format!(
            "1. 不要な変数であれば宣言を削除してください。\n\
            2. 意図的に未使用にする場合は、変数名の先頭にアンダースコアを付けてください（例: `_{}`）。",
            var_name
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

        jd.beginner_tip = Some(format!(
            "「変数を作ったけれど使っていません」。使わない場合は変数名を `_{}` にすると警告が消えます。",
            var_name
        ));
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
            "定義された変数が一度も使われていない場合に発生します。",
            "タイポや未使用リソースの検知が目的です。",
            "変数を削除するか、先頭に `_` を付けて `_var` と命名してください。",
        )
    }
}
