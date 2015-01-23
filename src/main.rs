use std::io;
use std::rand;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  let secret_number: uint = (rand::random::<uint>() % 100u) + 1u;

  println!("The secret number is: {}", secret_number);

  println!("Please input your guess.");

  let input = io::stdin().read_line().ok().expect("Failed to read line");
  let input_num: Option<uint> = input.trim().parse::<uint>();

  let num = match input_num {
    Some(num) => num,
    None      => {
      println!("Please input a number!");
      return;
    }
  };

  println!("You guessed: {}", num);

  match cmp(num, secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!")
  }
}

fn cmp(a: uint, b: uint) -> Ordering {
  if a < b { Ordering::Less }
  else if a > b  { Ordering::Greater }
  else { Ordering::Equal }
}
