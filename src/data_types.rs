pub fn run(test: &str) {
  println!("Hello");
  let string = String::from(test);
  println!("{}",string);
}

pub fn add(num1: i8, num2: i8) {
  let addition: i8 = num1 + num2;
  println!("{} + {} = {}",num1,num2,addition);
  run("From the add function!!");
}

pub fn cond() {
  let age: i8 = 19;
  if age <= 20 {
    println!("You are younger than 20");
  } else {
    println!("You are older than 20");
  }
}

//Function that returns something
pub fn returnNumber(number: i8) -> i8 {
  return number;
}
