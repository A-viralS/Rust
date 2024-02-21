fn main() {


//tuples
    let mut tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.2);
    tup.1 = false;
    println!("{}", tup.1);


    // arrays 
    let mut arr=[1,2,3,4,5];
    arr[4]=6;
    println!("{}",arr[4]);

}
