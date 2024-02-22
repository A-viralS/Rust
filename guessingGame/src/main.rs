use colored::*;
use rand::Rng;
use std::cmp::Ordering; // for comparing
use std::io;
fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is :{}", secret_number);
    loop {
        println!("please enter the number ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line ");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "you won :-)".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
