use super::types::{CargoMessage, Diagnostic};

pub fn parse_line(line: &str) -> Option<CargoMessage> {
    if !line.trim_start().starts_with('{') {
        return None;
    }
    serde_json::from_str::<CargoMessage>(line).ok()
}

#[allow(dead_code)]
pub fn extract_diagnostic(line: &str) -> Option<Diagnostic> {
    match parse_line(line)? {
        CargoMessage::CompilerMessage { message, .. } => Some(message),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_compiler_message() {
        let json = r#"{"reason":"compiler-message","package_id":"test 0.1.0","target":{},"message":{"message":"cannot assign to `a`, as it is not declared as mutable","code":{"code":"E0596","explanation":null},"level":"error","spans":[{"file_name":"src/main.rs","byte_start":25,"byte_end":26,"line_start":3,"line_end":3,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    a = 2;","highlight_start":5,"highlight_end":6}],"label":"cannot assign to immutable variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}}"#;
        let diag = extract_diagnostic(json).expect("Should parse diagnostic");
        assert_eq!(diag.code.as_ref().map(|c| c.code.as_str()), Some("E0596"));
        assert_eq!(diag.level, "error");
    }
}
