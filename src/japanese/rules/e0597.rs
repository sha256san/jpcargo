use regex::Regex;
use crate::diagnostic::{format_location, format_snippet, Diagnostic, ErrorCategory};
use crate::japanese::template::JapaneseDiagnostic;
use super::DiagnosticRule;

pub struct E0597;

impl DiagnosticRule for E0597 {
    fn code(&self) -> &'static str {
        "E0597"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Borrow
    }

    fn title(&self) -> &'static str {
        "借用先（参照されている値）の生存期間が短すぎます"
    }

    fn explain(&self, diag: &Diagnostic) -> JapaneseDiagnostic {
        let var_re = Regex::new(r"`(?P<var>[^`]+)` does not live long enough").unwrap();
        let var_name = var_re
            .captures(&diag.message)
            .and_then(|c| c.name("var"))
            .map(|m| m.as_str())
            .unwrap_or("該当の値");

        let summary = format!(
            "値「{}」は参照よりも早く破棄（ドロップ）されてしまうため、ダングリングポインタ（無効な参照）になる恐れがあります。",
            var_name
        );

        let reason = "Rust では参照（ポインタ）は必ず「参照先の実データ」よりも短いか等しい期間しか生存できません。\n\
            内側のスコープで生成されたローカル変数への参照を外側のスコープや戻り値に渡そうとすると、このエラーが発生します。";

        let solution = format!(
            "1. 参照される値「{}」の定義場所を外側のスコープに移動して生存期間を延ばすか、\n\
            2. 参照ではなく所有権（値そのもの）を渡すか、`.clone()` を使用してください。",
            var_name
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
            "参照されている値のライフタイムが、参照そのもののライフタイムより短いため発生します。",
            "破棄されたメモリへの不正アクセス（ダングリング参照）を防ぐための制約です。",
            "値の宣言スコープを広げるか、値そのものを所有権移動で返してください。",
        )
    }
}
