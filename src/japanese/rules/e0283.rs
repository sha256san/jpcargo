use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0283;

impl DiagnosticRule for E0283 {
    fn code(&self) -> &'static str {
        "E0283"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Trait
    }

    fn title(&self) -> &'static str {
        "Trait の実装型が一意に定まりません（型注釈が必要です）"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "該当の Trait を実装している型が複数存在するため、コンパイラがどの型として処理すればよいか決定できません。";

        let reason = "例えば `\"10\".parse()` のように、複数の型（`i32`, `u64`, `f32` 等）が同じ Trait を実装している場合、\n\
            受ける側の型が決まっていないとコンパイラは型を特定できません。";

        let solution = "1. 変数に明示的な型を付ける（例: `let x: i32 = ...;`）\n\
            2. ターボフィッシュ記法で型を指定する（例: `\"10\".parse::<i32>()`）\n\
            3. 完全修飾構文 `<Type as Trait>::method(...)` を使用する";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("`.parse()` などを呼ぶときは、`let x: i32 = ...` のように変数に型を書くか、`.parse::<i32>()` と書きましょう。".to_string());
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
            "Traitの実装候補が複数あり、コンパイラが型を特定できない場合に発生します。",
            "型の曖昧性を解消するための情報が不足しています。",
            "明示的な型注釈またはターボフィッシュ（`::<Type>`）で型を指定してください。",
        )
    }
}
