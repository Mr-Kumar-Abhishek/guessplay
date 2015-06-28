extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hurry ! guess a number between 1 and 100 quick !!");
    println!("Now quickly enter what you have guessed !");
    
    let mut myguess = String::new();

    io::stdin()
	.read_line(&mut myguess)
	.ok()
	.expect("can not read !! I am blind again !!");
    
    let myguess: u32 = myguess
	.trim()
	.parse()
	.ok()
	.expect("huh ? type a number");

    println!("You guessed: {}", myguess);
    
    let secret = rand::thread_rng().gen_range(1,101);
    match myguess.cmp(&secret) {
	Ordering::Less => println!("Too small !!"),
	Ordering::Greater => println!("Too BIG !!"),
	Ordering::Equal => println!("You Win !!!"),
    }

    println!("The secret number is: {}", secret);

}
