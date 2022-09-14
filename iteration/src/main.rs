fn main() {
  let numbers = [1,2,3,4,5];

  //also we can create the list with stating int types
  // let numbers: [i32 ; 5] = [1,2,3,4,5]; 


//normal iteration type in numbers list
  for n in numbers.iter(){
    println!("{}",n);
  }

//another iteration bound to length of array
for i in 0..numbers.len(){
    println!("*********");
    println!("{}", numbers[i]);
}

//if there is vector
let my_vec = vec!["dog","cat","lion"];

for (index,a) in my_vec.iter().enumerate(){
  println!("The animal {} at index:[{}] ",a,index)
}


//---------while loop-----

let mut counter = 5;
while counter !=0 {
  println!("The counter is {}",counter);
  counter -=1;
}




}
