use std::io;

fn main() {
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("expected to readline");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}",int_input + 2);


    //let x: u8 = 12; // 0-255 <u> means it is unsigned
    //let y: i8 = 12; // -128 - 127 <i> means it is signed, can get negatif values

    //let z = x + y // z value going to be error all type (x and y) must be the same 
}
