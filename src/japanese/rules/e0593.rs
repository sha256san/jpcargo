use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0593;

impl DiagnosticRule for E0593 {
    fn code(&self) -> &'static str {
        "E0593"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Closure
    }

    fn title(&self) -> &'static str {
        "クロージャの引数の個数が、要求されているトレイト（Fn/FnMut/FnOnce）と一致していません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "渡されたクロージャが受け取る引数の個数が、呼び出し側（イテレータの `.map()` や `.filter()` 等）が渡そうとしている引数の個数と合致していません。";
        let reason = "例えば `.map(|x, y| ...)` のように2個の引数を取ろうとしたが、イテレータの各要素は1個しか渡されない場合などに発生します。";
        let solution = "クロージャの引数定義を、要求されているシグネチャに合わせて修正してください（タプルの場合は `|(x, y)|` のようにパターンで分解します）。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("タプル `(a, b)` を受け取るクロージャは `|a, b|` ではなく `|(a, b)|` と丸括弧で囲んで書きます。".to_string());
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
            "クロージャが受け取る引数の数が、要求されるクロージャトレイトと一致しない場合に発生します。",
            "イテレータメソッドなどでの引数指定ミスが原因です。",
            "引数の個数やタプルパターン（`|(a, b)|`）を確認してください。",
        )
    }
}
