use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::{FixOption, JapaneseDiagnostic};
use super::DiagnosticRule;

pub struct UnusedAssignments;

impl DiagnosticRule for UnusedAssignments {
    fn code(&self) -> &'static str {
        "unused_assignments"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lint
    }

    fn title(&self) -> &'static str {
        "代入された値が一度も使われないまま上書き（再代入）されています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"value assigned to `(?P<var>[^`]+)` is never read").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("var");

        let summary = format!(
            "変数「{}」に値を代入しましたが、その値が読み取られる前に別の値が代入されているか、使われないままスコープが終了しています。",
            var_name
        );

        let reason = "無駄な初期化や不要な再代入が行われており、パフォーマンスや可読性の低下の原因となるため、コンパイラが警告を出しています。";
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
            "方法1: 不要な初期化を削除し、最初から最終的な値を代入する",
            format!("let mut {} = 1; {} = 2;", var_name, var_name),
            format!("let {} = 2;", var_name),
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: 不要な再代入コードを削除する",
            format!("{} = 2;", var_name),
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
            "変数に代入された値が一度も読み取られないまま再代入・破棄された場合に発生します。",
            "不要な初期化・無駄な処理を防ぐための品質検査です。",
            "",
        );
        jd.add_fix_option(FixOption::diff(
            "方法1: 最初から最終的な値を代入する",
            "let mut x = 1; x = 2;",
            "let x = 2;",
        ));
        jd.add_fix_option(FixOption::diff(
            "方法2: 不要な再代入を削除する",
            "x = 2;",
            "// (削除)",
        ));
        jd
    }
}
