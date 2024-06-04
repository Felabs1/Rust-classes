
//Rust supports typical control flow constructs like if, else, and loop.
fn main() {
    let number = 10;

    if number < 5 {
        println!("Less than 5");
    } else if number == 5 {
        println!("Equal to 5");
    } else {
        println!("Greater than 5");
    }

    for i in 1..4 {
        println!("i = {}", i);
    }

    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
}
