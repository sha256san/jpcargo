use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0638;

impl DiagnosticRule for E0638 {
    fn code(&self) -> &'static str {
        "E0638"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "This error indicates that the struct, enum or enum variant must be matched"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let mut jd = self.general_explanation();
        jd.level = diag.level.clone();
        jd.location = format_location(diag);
        jd.snippet = format_snippet(diag);
        jd.original_message = Some(diag.message.clone());

        for child in &diag.children {
            jd.suggestions.push(format!("{}: {}", child.level, child.message));
        }

        jd
    }

    fn general_explanation(&self) -> JapaneseDiagnostic {
        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            "error",
            self.title(),
            "This error indicates that the struct, enum or enum variant must be matched non-exhaustively as it has been marked as non_exhaustive.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "#[non_exhaustive]\npub enum Error {\n    Message(String),\n    Other,\n}\n\nimpl Display for Error {\n    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {\n        // This will not error, despite being marked as non_exhaustive, as this\n        // enum is defined within the current crate, it can be matched\n        // exhaustively.\n        let display = match self {\n            Message(s) => s,\n            Other => \"other or unknown error\",\n        };\n        formatter.write_str(display)\n    }\n}"));

        jd
    }
}
