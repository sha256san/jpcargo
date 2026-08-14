use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0521;

impl DiagnosticRule for E0521 {
    fn code(&self) -> &'static str {
        "E0521"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Closure
    }

    fn title(&self) -> &'static str {
        "借用したデータがクロージャ（またはスレッド）の外側に漏れ出そうとしています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "クロージャ内でキャプチャした参照の生存期間が、クロージャ自体の要求されるライフタイム（またはスレッドの `'static` 境界）よりも短いため発生しました。";
        let reason = "`std::thread::spawn` などはキャプチャする変数が `'static`（プログラム終了まで生存可能）であることを要求します。ローカル変数の参照を渡すと、スレッド実行中に変数が破棄されてダングリングポインタが生じる恐れがあります。";
        let solution = "1. `move` キーワードをクロージャの前に付けて、所有権ごとクロージャ内に移動させてください（例: `move || { ... }`）。\n2. 参照ではなく `.clone()` した値を渡してください。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("スレッド（`thread::spawn`）にデータを渡すときは、`move || { ... }` を使って所有権ごとスレッドに渡すのが基本です。".to_string());
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
            "借用データがクロージャやスレッドの生存期間制約（`'static` 等）を満たさない場合に発生します。",
            "スレッド実行中にスタックデータが破棄されるのを防ぐための制約です。",
            "`move` クロージャにして所有権を渡してください。",
        )
    }
}
