fn main(){
//Alternatively it calculates the area of a rectangle
// different approach from function in main2.rs file
let rect: (u32,u32) = (30,50);

println!("The area of rectangle {}", area(rect));

}

fn area(dimensions:(u32,u32)) -> u32 {
dimensions.0 * dimensions.1  //returns
}
