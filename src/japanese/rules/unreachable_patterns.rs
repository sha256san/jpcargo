use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct UnreachablePatterns;

impl DiagnosticRule for UnreachablePatterns {
    fn code(&self) -> &'static str {
        "unreachable_patterns"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Pattern
    }

    fn title(&self) -> &'static str {
        "先行するパターンによってすべてカバーされており、このアームには絶対にマッチしません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "`match` 式などで、上のパターン（ワイルドカード `_` や広範なガード）ですでにすべての値が捕捉されているため、このパターンが評価されることはありません。";
        let reason = "`match` 式は上から順に評価されるため、包括的なパターンを上に書くと下のパターンが到達不能になります。";
        let solution = "1. より具体的なパターンを上に配置するように順序を入れ替えてください。\n\
            2. 不要になった余分な match アームを削除してください。";

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
            "上位の match パターンですでに捕捉されている場合に発生します。",
            "match 式の上から順次評価ルールによるものです。",
            "具体的なパターンを上に並べるか、余分なアームを削除してください。",
        )
    }
}
