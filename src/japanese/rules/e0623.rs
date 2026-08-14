use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0623;

impl DiagnosticRule for E0623 {
    fn code(&self) -> &'static str {
        "E0623"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Lifetime
    }

    fn title(&self) -> &'static str {
        "A lifetime didn't match what was expected"
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
            "A lifetime didn't match what was expected. Erroneous code example:",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct Foo<'a, 'b, T>(std::marker::PhantomData<(&'a (), &'b (), T)>)\nwhere\n    T: Convert<'a, 'b>;\n\ntrait Convert<'a, 'b>: Sized {\n    fn cast(&'a self) -> &'b Self;\n}\nimpl<'long: 'short, 'short, T> Convert<'long, 'short> for T {\n    fn cast(&'long self) -> &'short T {\n        self\n    }\n}\n// error\nfn badboi<'in_, 'out, T>(\n    x: Foo<'in_, 'out, T>,\n    sadness: &'in_ T\n) -> &'out T {\n    sadness.cast()\n}"));

        jd
    }
}
