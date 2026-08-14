use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0432;

impl DiagnosticRule for E0432 {
    fn code(&self) -> &'static str {
        "E0432"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Module
    }

    fn title(&self) -> &'static str {
        "インポート（use）しようとした項目が見つかりません"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let import_re = Regex::new(r"unresolved import `(?P<item>[^`]+)`").unwrap();
        let item_name = import_re
            .captures(&diag.message)
            .and_then(|c| c.name("item"))
            .map(|m| m.as_str())
            .unwrap_or("指定の項目");

        let summary = format!(
            "`use {}` でインポートしようとした項目（モジュール・構造体・関数・トレイト等）が存在しません。",
            item_name
        );

        let reason = "指定されたパスが存在しないか、依存クレートが `Cargo.toml` に追加されていない、\n\
            あるいは対象の項目が `pub` で公開されていない可能性があります。";

        let solution = format!(
            "1. インポートパス「{}」の綴りを確認してください。\n\
            2. 外部クレートの場合は、`Cargo.toml` に依存関係が正しく追加されているか確認してください。\n\
            3. 自作モジュールの場合は、該当ファイルで `pub` キーワードが付いているか確認してください。",
            item_name
        );

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
            "`use` 文で指定したモジュールや型が存在しない場合に発生します。",
            "パスの間違い、依存関係の指定漏れ、非公開項目へのアクセスが原因です。",
            "モジュールパスや `Cargo.toml` を確認してください。",
        )
    }
}
