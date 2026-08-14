use super::types::Diagnostic;

/// Primary Span の位置文字列表現 (例: "src/main.rs:3:5") を取得
pub fn format_location(diag: &Diagnostic) -> Option<String> {
    diag.spans
        .iter()
        .find(|s| s.is_primary)
        .or_else(|| diag.spans.first())
        .map(|s| format!("{}:{}:{}", s.file_name, s.line_start, s.column_start))
}

/// Primary Span のコードスニペットとポインタ行を生成
pub fn format_snippet(diag: &Diagnostic) -> Option<String> {
    let span = diag
        .spans
        .iter()
        .find(|s| s.is_primary)
        .or_else(|| diag.spans.first())?;

    if span.text.is_empty() {
        return None;
    }

    let mut lines = Vec::new();
    let line_start = span.line_start;

    for (idx, span_text) in span.text.iter().enumerate() {
        let current_line = line_start + idx;
        lines.push(format!("{:4} | {}", current_line, span_text.text));

        // 該当箇所のアンダーライン (^ 記号) を生成
        let highlight_len = if span_text.highlight_end > span_text.highlight_start {
            span_text.highlight_end - span_text.highlight_start
        } else {
            1
        };

        let pointer_indent = " ".repeat(span_text.highlight_start.saturating_sub(1));
        let pointer = "^".repeat(highlight_len);
        let label = span
            .label
            .as_deref()
            .map(|l| format!(" {}", l))
            .unwrap_or_default();

        lines.push(format!("     | {}{}{}", pointer_indent, pointer, label));
    }

    Some(lines.join("\n"))
}
