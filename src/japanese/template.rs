use crate::diagnostic::ErrorCategory;

/// 修正方法とコード例のセット
#[derive(Debug, Clone)]
pub struct FixOption {
    /// 日本語の一言コメント（例: "方法1: 所有権を渡す代わりに参照（借用）を渡す"）
    pub description: String,
    /// 修正前後の Diff（- before, + after）
    pub diff: Option<(String, String)>,
    /// 単体の修正コード例
    pub code: Option<String>,
}

#[allow(dead_code)]
impl FixOption {
    pub fn diff(description: impl Into<String>, before: impl Into<String>, after: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            diff: Some((before.into(), after.into())),
            code: None,
        }
    }

    pub fn code(description: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            diff: None,
            code: Some(code.into()),
        }
    }

    pub fn comment_only(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            diff: None,
            code: None,
        }
    }
}

/// レンダラーに渡される整形済みの日本語診断情報
#[derive(Debug, Clone)]
pub struct JapaneseDiagnostic {
    pub code: String,
    pub category: ErrorCategory,
    pub level: String,
    pub title: String,
    pub summary: String,
    pub location: Option<String>,
    pub snippet: Option<String>,
    pub reason: String,
    pub solution: String,
    pub beginner_tip: Option<String>,
    pub expert_note: Option<String>,
    pub example_diff: Option<(String, String)>, // (before, after)
    pub fix_options: Vec<FixOption>,            // 複数の修正方法とコード例
    pub suggestions: Vec<String>,
    pub original_message: Option<String>,
}

impl JapaneseDiagnostic {
    pub fn new(
        code: impl Into<String>,
        category: ErrorCategory,
        level: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        reason: impl Into<String>,
        solution: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            category,
            level: level.into(),
            title: title.into(),
            summary: summary.into(),
            location: None,
            snippet: None,
            reason: reason.into(),
            solution: solution.into(),
            beginner_tip: None,
            expert_note: None,
            example_diff: None,
            fix_options: Vec::new(),
            suggestions: Vec::new(),
            original_message: None,
        }
    }

    pub fn add_fix_option(&mut self, option: FixOption) {
        self.fix_options.push(option);
    }
}
