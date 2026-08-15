use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
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
            .unwrap_or("var");

        let summary = format!(
            "変数「{}」が定義されましたが、以降のコードで一度も読み取られていません。",
            var_name
        );

        let reason = "タイポやロジックの書き忘れを防ぐため、Rustコンパイラは未使用変数を警告として検知します。";
        let solution = "";

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

        // 複数の修正方法とコード例（日本語コメント付き）
        jd.add_fix_option(FixOption::diff(
            "方法1: 意図的に未使用にする場合は先頭にアンダースコアを付ける",
            format!("let {} = ...;", var_name),
            format!("let _{} = ...;", var_name),
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: 不要な変数であれば宣言そのものを削除する",
            format!("let {} = ...;", var_name),
            "// (削除)",
        ));

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "warning",
            self.title(),
            "定義された変数が一度も使われていない場合に発生します。",
            "タイポや未使用リソースの検知が目的です。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: 先頭にアンダースコアを付ける",
            "let x = ...;",
            "let _x = ...;",
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: 不要な変数を削除する",
            "let x = ...;",
            "// (削除)",
        ));
        jd
    }
}
