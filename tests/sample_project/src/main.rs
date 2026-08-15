fn test_ownership() {
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s);
}

fn test_mutability() {
    let x = 10;
    x = 20;
}

fn test_type_mismatch() {
    let num: u32 = "42";
}

fn test_borrow() {
    let mut list = vec![1, 2, 3];
    let first = &list[0];
    list.push(4);
    println!("{}", first);
}

fn main() {
    test_ownership();
    test_mutability();
    test_type_mismatch();
    test_borrow();
}