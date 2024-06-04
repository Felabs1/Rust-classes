fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
