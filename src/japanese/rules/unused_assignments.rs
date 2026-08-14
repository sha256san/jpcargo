use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
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
            .unwrap_or("該当の変数");

        let summary = format!(
            "変数「{}」に値を代入しましたが、その値が読み取られる前に別の値が代入されているか、使われないままスコープが終了しています。",
            var_name
        );

        let reason = "無駄な初期化や不要な再代入が行われており、パフォーマンスや可読性の低下の原因となるため、コンパイラが警告を出しています。";
        let solution = format!(
            "不要な代入（例: `let mut {} = 1;` 直後の `{} = 2;`）を削除するか、初期化時の値を直接指定してください（例: `let mut {} = 2;`）。",
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

        jd.beginner_tip = Some(format!(
            "「最初に代入した値」が一度も読まれないまま上書きされています。初期値を最初から必要な値にするか、不要な代入を消しましょう。"
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
            "変数に代入された値が一度も読み取られないまま再代入・破棄された場合に発生します。",
            "不要な初期化・無駄な処理を防ぐための品質検査です。",
            "不要な代入を削除するか、初期化時の代入値を正しく設定してください。",
        )
    }
}
