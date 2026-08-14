use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0282;

impl DiagnosticRule for E0282 {
    fn code(&self) -> &'static str {
        "E0282"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Type
    }

    fn title(&self) -> &'static str {
        "コンパイラが型を推論できません（明示的な型注釈が必要です）"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "コンパイラに十分な型情報が与えられていないため、変数の型を自動推論できませんでした。";

        let reason = "Rust は高度な型推論機能を備えていますが、関数の戻り値が複数の型を取り得る場合（例: `.collect()`, `.parse()` 等）や、\n\
            初期値がなく後からも型情報が得られない場合、推論を完結できません。";

        let solution = "変数宣言に明示的な型を付けるか（例: `let x: Vec<i32> = ...;`）、\n\
            ターボフィッシュ記法（例: `.collect::<Vec<_>>()` または `.parse::<i32>()`）で型を指定してください。";

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
            "error",
            self.title(),
            "型の推論に必要な情報が不足している場合に発生します。",
            "ジェネリックメソッドの型が曖昧な場合に明示が必要です。",
            "`let x: Type = ...` または `.method::<Type>()` で型注釈を与えてください。",
        )
    }
}
