use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0158;

impl DiagnosticRule for E0158 {
    fn code(&self) -> &'static str {
        "E0158"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "A generic parameter or static has been referenced in a pattern"
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
            "A generic parameter or static has been referenced in a pattern. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "enum Foo {\n    One,\n    Two\n}\n\ntrait Bar {\n    const X: Foo;\n}\n\nfn test<A: Bar>(arg: Foo) {\n    match arg {\n        A::X => println!(\"A::X\"), // error: E0158: constant pattern depends\n                                  //        on a generic parameter\n        Foo::Two => println!(\"Two\")\n    }\n}"));

        jd
    }
}
