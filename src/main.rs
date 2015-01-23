use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  let input = io::stdn().read_line().ok().expect("Failed to read line");

  println!("You guessed: {}", input);
}
