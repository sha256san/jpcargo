use crate::diagnostic::ErrorCategory;

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
            suggestions: Vec::new(),
            original_message: None,
        }
    }
}
