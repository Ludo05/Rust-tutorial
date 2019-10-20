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
