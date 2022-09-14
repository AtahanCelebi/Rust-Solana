fn main() {
    print_number(10); //iteration to num showing is even or odd
}


fn print_number(num: u32){
    for n in 0..num+1{
        //println!("{}",n);
        if is_even(n){
            println!("{} is even!",n)
        }
        else{
            println!("{} is odd",n)
        }
    }
}


fn is_even(num: u32) -> bool {
    return num %2==0;
}
