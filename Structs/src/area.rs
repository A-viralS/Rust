struct Rectangle{
    width:u64,
    height:u64
}
fn main (){
let rect=  Rectangle{
width:30,
height:50
};
println!("the area of rectangle is {} square pixels", area(&rect) );

}

fn area(rectangle:&Rectangle)->u64{
    rectangle.width*rectangle.height
}