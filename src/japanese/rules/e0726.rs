use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0726;

impl DiagnosticRule for E0726 {
    fn code(&self) -> &'static str {
        "E0726"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "An argument lifetime was elided in an async function"
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
            "An argument lifetime was elided in an async function. Erroneous code example:",
            "参照の有効期間（ライフタイム）が参照先データの生存期間を超えないようにする制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "use futures::executor::block_on;\nstruct Content<'a> {\n    title: &'a str,\n    body: &'a str,\n}\nasync fn create(content: Content) { // error: implicit elided\n                                    // lifetime not allowed here\n    println!(\"title: {}\", content.title);\n    println!(\"body: {}\", content.body);\n}\nlet content = Content { title: \"Rust\", body: \"is great!\" };\nlet future = create(content);\nblock_on(future);"));

        jd
    }
}
