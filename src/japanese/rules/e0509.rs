use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0509;

impl DiagnosticRule for E0509 {
    fn code(&self) -> &'static str {
        "E0509"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Ownership
    }

    fn title(&self) -> &'static str {
        "This error occurs when an attempt is made to move out of a value whose type"
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
            "This error occurs when an attempt is made to move out of a value whose type implements the Drop trait.",
            "match式の網羅性やパターンバインディングの規則による制約です。",
            "コンパイラのエラーメッセージおよびヒント（help/note）に従って、該当箇所のコードを修正してください。",
        );

        

        jd.suggestions.push(format!("コード例:\n{}", "struct FancyNum {\n    num: usize\n}\n\nstruct DropStruct {\n    fancy: FancyNum\n}\n\nimpl Drop for DropStruct {\n    fn drop(&mut self) {\n        // Destruct DropStruct, possibly using FancyNum\n    }\n}\n\nfn main() {\n    let drop_struct = DropStruct{fancy: FancyNum{num: 5}};\n    let fancy_field = drop_struct.fancy; // Error E0509\n    println!(\"Fancy: {}\", fancy_field.num);\n    // implicit call to `drop_struct.drop()` as drop_struct goes out of scope\n}"));

        jd
    }
}
