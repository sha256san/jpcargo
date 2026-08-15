use std::process::Command;

#[test]
fn test_jpcargo_cli_list() {
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute jpcargo list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("E0596"));
    assert!(stdout.contains("E0308"));
    assert!(stdout.contains("E0382"));
    assert!(stdout.contains("E0502"));
    assert!(stdout.contains("E0499"));
    assert!(stdout.contains("E0505"));
    assert!(stdout.contains("E0507"));
    assert!(stdout.contains("E0599"));
    assert!(stdout.contains("E0072"));
    assert!(stdout.contains("E0133"));
}

#[test]
fn test_jpcargo_cli_stats() {
    let output = Command::new("cargo")
        .args(&["run", "--", "stats"])
        .output()
        .expect("Failed to execute jpcargo stats");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("対応エラー総数: 528 件"));
    assert!(stdout.contains("所有権 (Ownership)"));
    assert!(stdout.contains("借用 (Borrow)"));
    assert!(stdout.contains("トレイト (Trait)"));
    assert!(stdout.contains("可変性 (Mutability)"));
    assert!(stdout.contains("未使用・コード品質 (Lint)"));
}

#[test]
fn test_jpcargo_cli_explain_unused_assignments() {
    let output = Command::new("cargo")
        .args(&["run", "--", "explain", "unused_assignments"])
        .output()
        .expect("Failed to execute jpcargo explain unused_assignments");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unused_assignments"));
    assert!(stdout.contains("代入された値"));
}

#[test]
fn test_jpcargo_cli_explain_e0806() {
    let output = Command::new("cargo")
        .args(&["run", "--", "explain", "E0806"])
        .output()
        .expect("Failed to execute jpcargo explain E0806");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("E0806"));
}

#[test]
fn test_jpcargo_cli_explain_e0596() {
    let output = Command::new("cargo")
        .args(&["run", "--", "explain", "E0596"])
        .output()
        .expect("Failed to execute jpcargo explain");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("E0596"));
    assert!(stdout.contains("ミュータブル"));
    assert!(stdout.contains("let mut"));
}

#[test]
fn test_jpcargo_cli_explain_e0599() {
    let output = Command::new("cargo")
        .args(&["run", "--", "explain", "E0599"])
        .output()
        .expect("Failed to execute jpcargo explain E0599");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("E0599"));
    assert!(stdout.contains("メソッド"));
}

#[test]
fn test_jpcargo_cli_explain_e0072() {
    let output = Command::new("cargo")
        .args(&["run", "--", "explain", "E0072"])
        .output()
        .expect("Failed to execute jpcargo explain E0072");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("E0072"));
    assert!(stdout.contains("無限大"));
    assert!(stdout.contains("Box<"));
}

#[test]
fn test_jpcargo_cli_search() {
    let output = Command::new("cargo")
        .args(&["run", "--", "search", "borrow"])
        .output()
        .expect("Failed to execute jpcargo search");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("E0499"));
    assert!(stdout.contains("E0502"));
}

#[test]
fn test_jpcargo_cli_doctor() {
    let output = Command::new("cargo")
        .args(&["run", "--", "doctor"])
        .output()
        .expect("Failed to execute jpcargo doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rust & Cargo 開発環境診断"));
    assert!(stdout.contains("Rust コンパイラ"));
    assert!(stdout.contains("Cargo パッケージマネージャ"));
}
