use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0133;

impl DiagnosticRule for E0133 {
    fn code(&self) -> &'static str {
        "E0133"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Unsafe
    }

    fn title(&self) -> &'static str {
        "unsafe な関数や操作を、通常の（安全な）コンテキストから呼び出しています"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let summary = "呼び出そうとしている関数・操作には `unsafe` が付与されていますが、呼び出し側が `unsafe` ブロックで囲まれていません。";

        let reason = "Rust では、生ポインタの参照外し、FFI（C言語関数呼び出し）、可変静的変数（`static mut`）へのアクセスなど、\n\
            コンパイラがメモリ安全性を自動検証できない操作には `unsafe` キーワードが要求されます。\n\
            開発者が明示的に「安全性を検証済みである」と責任を持つ必要があります。";

        let solution = "1. `unsafe { ... }` ブロックで対象の呼び出しを囲んでください。\n\
            2. 関数のシグネチャ自体を `unsafe fn ...` に変更してください。\n\
            3. 【注意】単に `unsafe` を付けるだけでなく、メモリ破壊や未定義動作（UB）が起きない前提条件が満たされているか必ず確認してください。";

        let mut jd = JapaneseDiagnostic::new(
            self.code(),
            self.category(),
            &diag.level,
            self.title(),
            summary,
            reason,
            solution,
        );

        jd.beginner_tip = Some("初心者のうちは、できるだけ `unsafe` を使わず、安全な標準ライブラリの関数で代用できないか検討しましょう。".to_string());
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
            "非安全（unsafe）な関数や操作を、unsafe ブロックの外から直接実行しようとした場合に発生します。",
            "コンパイラによる自動安全保証の範囲外であるため、明示的な `unsafe` 宣言が必要です。",
            "`unsafe { ... }` ブロックで囲むか、安全な代替手段を利用してください。",
        )
    }
}
