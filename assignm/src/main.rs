use std::io::{self, Read};

fn main() {
  let mut input_str = String::new();
  println!("Enter an integer: ");

  // Read user input into a String
  io::stdin().read_to_string(&mut input_str).expect("Error reading input");

  // Parse the String into an integer
  let mut input: i32 = match input_str.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid input. Please enter an integer.");
      return;
    }
  };



  // Print from 0 to input using a for loop
  for i in 0..=input {
    println!("{}", i);
  }
}
