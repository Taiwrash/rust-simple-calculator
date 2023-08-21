use std::env::{args, Args};

fn main() {
  let mut args:Args = args();
  let first = args.nth(1).unwrap();
  let ops = args.nth(0).unwrap();
  let second = args.nth(0).unwrap();

  // parsing string to f32
  let firstf32 = first.parse::<f32>().unwrap();
  let secondf32 = second.parse::<f32>().unwrap();
  let opschar = ops.parse::<char>().unwrap();
  let result = operate(opschar, firstf32, secondf32);
  println!("{:?}", formated_output(firstf32, opschar, secondf32, result));
}

fn operate(ops: char, firstf32:f32, secondf32: f32)->f32{
match ops{
'+'=>firstf32 + secondf32,
'-'=>firstf32 - secondf32,
'*' | 'x' | 'X'=>firstf32 * secondf32,
'/'=>firstf32 + secondf32,
_=> panic!("That is not an operator")
}
}

fn formated_output(firstf32:f32, ops:char, secondf32: f32, result:f32) -> String{
  format!("{:?} {} {} = {}", firstf32, ops, secondf32, result)
}