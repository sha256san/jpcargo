use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct UnusedImports;

impl DiagnosticRule for UnusedImports {
    fn code(&self) -> &'static str {
        "unused_imports"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "`use` でインポートされたモジュールや型が一度も使われていません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let import_re = Regex::new(r"unused import: `(?P<imp>[^`]+)`").unwrap();
        let import_name = import_re
            .captures(&diag.message)
            .and_then(|c| c.name("imp"))
            .map(|m| m.as_str())
            .unwrap_or("該当のインポート");

        let summary = format!(
            "`use` 文でインポートした「{}」は、ファイル内で一度も使用されていません。",
            import_name
        );

        let reason = "不要なインポートはスコープを汚染し、名前衝突の原因となるため、コンパイラが警告を出します。";
        let solution = format!("不要な `use {}` を削除するか、コメントアウトしてください。", import_name);

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
            "`use` でインポートした型や関数が一度も参照されていない場合に発生します。",
            "コード整理と名前空間の清潔化が目的です。",
            "不要な `use` 行を削除してください。",
        )
    }
}
