use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    Mutability,
    Ownership,
    Borrow,
    Lifetime,
    Type,
    Trait,
    NameResolution,
    Function,
    Struct,
    Pattern,
    Module,
    Visibility,
    Operator,
    Closure,
    Async,
    Const,
    Unsafe,
    Macro,
    Feature,
    Memory,
    Generic,
    Lint,
    Style,
    Other,
}

impl ErrorCategory {
    pub fn name_ja(&self) -> &'static str {
        match self {
            ErrorCategory::Mutability => "可変性 (Mutability)",
            ErrorCategory::Ownership => "所有権 (Ownership)",
            ErrorCategory::Borrow => "借用 (Borrow)",
            ErrorCategory::Lifetime => "ライフタイム (Lifetime)",
            ErrorCategory::Type => "型システム (Type)",
            ErrorCategory::Trait => "トレイト (Trait)",
            ErrorCategory::NameResolution => "名前解決 (Name Resolution)",
            ErrorCategory::Function => "関数 (Function)",
            ErrorCategory::Struct => "構造体 (Struct)",
            ErrorCategory::Pattern => "パターンマッチ (Pattern Match)",
            ErrorCategory::Module => "モジュール・インポート (Module/Import)",
            ErrorCategory::Visibility => "可視性 (Visibility)",
            ErrorCategory::Operator => "演算子 (Operator)",
            ErrorCategory::Closure => "クロージャ (Closure)",
            ErrorCategory::Async => "非同期処理 (Async)",
            ErrorCategory::Const => "定数評価 (Const)",
            ErrorCategory::Unsafe => "アンセーフ (Unsafe)",
            ErrorCategory::Macro => "マクロ (Macro)",
            ErrorCategory::Feature => "機能フラグ (Feature)",
            ErrorCategory::Memory => "メモリ・ライフサイクル (Memory/Drop)",
            ErrorCategory::Generic => "ジェネリクス (Generic)",
            ErrorCategory::Lint => "未使用・コード品質 (Lint)",
            ErrorCategory::Style => "命名規約・スタイル (Style)",
            ErrorCategory::Other => "その他 (Other)",
        }
    }
}

pub fn classify(code: &str) -> ErrorCategory {
    match code {
        "E0384" | "E0596" | "E0594" => ErrorCategory::Mutability,
        "E0382" | "E0505" | "E0507" | "E0161" => ErrorCategory::Ownership,
        "E0499" | "E0502" | "E0506" => ErrorCategory::Borrow,
        "E0597" | "E0106" | "E0621" | "E0495" => ErrorCategory::Lifetime,
        "E0308" | "E0282" | "E0606" | "E0614" | "E0608" | "E0391" => ErrorCategory::Type,
        "E0277" | "E0283" | "E0271" | "E0119" | "E0117" | "E0210" | "E0599" => ErrorCategory::Trait,
        "E0207" => ErrorCategory::Generic,
        "E0425" | "E0433" => ErrorCategory::NameResolution,
        "E0432" | "E0428" | "E0252" | "E0255" => ErrorCategory::Module,
        "E0061" | "E0062" | "E0069" => ErrorCategory::Function,
        "E0063" | "E0609" | "E0072" => ErrorCategory::Struct,
        "E0004" | "E0027" | "E0026" => ErrorCategory::Pattern,
        "E0603" | "E0616" => ErrorCategory::Visibility,
        "E0369" | "E0368" | "E0600" => ErrorCategory::Operator,
        "E0521" | "E0593" => ErrorCategory::Closure,
        "E0728" => ErrorCategory::Async,
        "E0015" | "E0080" => ErrorCategory::Const,
        "E0133" => ErrorCategory::Unsafe,
        "E0040" => ErrorCategory::Memory,
        "E0659" => ErrorCategory::Macro,
        "E0658" => ErrorCategory::Feature,
        "unused_assignments" | "unused_variables" | "unused_mut" | "dead_code" | "unused_imports"
        | "unused_must_use" | "unused_parens" | "unused_doc_comments" | "unused_comparisons"
        | "redundant_semicolons" | "unreachable_code" | "unreachable_patterns" | "while_true"
        | "bare_trait_objects" | "deprecated" => ErrorCategory::Lint,
        "non_snake_case" | "non_camel_case_types" | "non_upper_case_globals" => ErrorCategory::Style,
        _ => {
            if code.starts_with("clippy::") || code.starts_with("unused_") {
                ErrorCategory::Lint
            } else if let Some(entry) = crate::japanese::database::find_db_entry(code) {
                entry.category
            } else {
                ErrorCategory::Other
            }
        }
    }
}
