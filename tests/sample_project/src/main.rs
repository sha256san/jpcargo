fn test_ownership_error() {
    let s = String::from("hello");
    let s2 = s; // s の所有権が s2 にムーブする
    println!("{}", s); // E0382: ムーブ後の s を使用しようとしてエラー
}

fn test_mutability_error() {
    let x = 100;
    x += 50; // E0384: 不変変数 x を変更しようとしてエラー
}

fn test_type_mismatch() {
    let num: u32 = "123"; // E0308: &str を u32 に代入しようとして型不一致エラー
}

fn test_borrow_error() {
    let mut vec = vec![1, 2, 3];
    let first = &vec[0]; // 不変借用
    vec.push(4); // E0502: 不変借用中に可変借用（push）を行ってエラー
    println!("First: {}", first);
}

fn main() {
    test_ownership_error();
    test_mutability_error();
    test_type_mismatch();
    test_borrow_error();
}
//bad code