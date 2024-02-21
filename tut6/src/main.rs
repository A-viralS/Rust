use std::io ;

fn main() {
    let x: i32 = 5;
    let y: i32 = 8;
    let z = x + y;
    println!("{}", z);
    let a = 9i8;
    let b = 5i16;
    let z = a + b as i8;
    println!("{}", z);
    //convert string to number
let mut input =String::new();
io::stdin().read_line(&mut input).expect("input not read");
let int_input:i64=input.trim().parse().unwrap();
println!("{}",int_input+2);

}
