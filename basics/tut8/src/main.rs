use std::io;
fn main() {
    println!("enter the numbers ");
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1).unwrap();
    let int_n1: i32 = n1.trim().parse().unwrap();
    let mut n2 = String::new();
    io::stdin().read_line(&mut n2).unwrap();
    let int_n2: i32 = n2.trim().parse().unwrap();

    println!("first number {}", n1);
    println!("second number {}", n2);
    let result = add_numbers(int_n1, int_n2);
    println!("Result {}", result);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let sum = x + y;
    if sum > 10 {
        return sum * 100; // to return previously use the return keyword
    }

    sum //just write without semicolon to return the result
}
