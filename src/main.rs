use std::io;
use std::rand;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  let secret_number: usize = (rand::random::<usize>() % 100us) + 1us;

  loop {
    println!("Please input your guess.");

    let input = io::stdin().read_line().ok().expect("Failed to read line");
    let input_num: Option<usize> = input.trim().parse::<usize>();

    let num = match input_num {
      Some(num) => num,
      None      => {
        println!("Please input a number!");
        continue;
      }
    };

    println!("You guessed: {}", num);

    match cmp(num, secret_number) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win!");
        return;
      }
    }
  }
}

fn cmp(a: usize, b: usize) -> Ordering {
  if a < b { Ordering::Less }
  else if a > b  { Ordering::Greater }
  else { Ordering::Equal }
}
