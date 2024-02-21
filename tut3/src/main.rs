fn main() {
    let x = 4;
    println!("the value of x is : {}", x);
    {
        //this is called name shadowing
        let x = 2;
        println!("the value of x is : {}", x);
    }
    let x = 5;
    println!("the value of x is : {}", x);

    let mut y = 6;
    println!("the value of y is : {}", y);
    y = 9;
    println!("the value of y is : {}", y);

    const THIS_IS_HOW_YOU_WRITE_CONSTANT: u32 = 60;
    println!(
        "the value of constant  is : {}",
        THIS_IS_HOW_YOU_WRITE_CONSTANT
    );
}
