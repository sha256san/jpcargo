use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0309;

impl DiagnosticRule for E0309 {
    fn code(&self) -> &'static str {
        "E0309"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "A parameter type is missing an explicit lifetime bound and may not live long"
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
            "A parameter type is missing an explicit lifetime bound and may not live long enough.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "// This won't compile because the applicable impl of\n// `SomeTrait` (below) requires that `T: 'a`, but the struct does\n// not have a matching where-clause.\nstruct Foo<'a, T> {\n    foo: <T as SomeTrait<'a>>::Output,\n}\n\ntrait SomeTrait<'a> {\n    type Output;\n}\n\nimpl<'a, T> SomeTrait<'a> for T\nwhere\n    T: 'a,\n{\n    type Output = u32;\n}"));

        jd
    }
}
