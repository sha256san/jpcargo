use serde::{Deserialize, Serialize};

/// Cargoの `--message-format=json` が出力する1行ごとのメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "reason")]
pub enum CargoMessage {
    #[serde(rename = "compiler-artifact")]
    CompilerArtifact {
        package_id: String,
        target: serde_json::Value,
    },
    #[serde(rename = "compiler-message")]
    CompilerMessage {
        package_id: String,
        target: serde_json::Value,
        message: Diagnostic,
    },
    #[serde(rename = "build-script-executed")]
    BuildScriptExecuted {
        package_id: String,
    },
    #[serde(rename = "build-finished")]
    BuildFinished {
        success: bool,
    },
    #[serde(other)]
    Unknown,
}

/// rustcの診断情報構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: String,
    #[serde(default)]
    pub spans: Vec<DiagnosticSpan>,
    #[serde(default)]
    pub children: Vec<DiagnosticChild>,
    pub rendered: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCode {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    #[serde(default)]
    pub text: Vec<SpanText>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
    pub suggestion_applicability: Option<String>,
    pub expansion: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanText {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticChild {
    pub message: String,
    pub level: String,
    #[serde(default)]
    pub spans: Vec<DiagnosticSpan>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
}
