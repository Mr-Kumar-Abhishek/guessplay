use std::io;

fn main() {
    println!("Hurry ! guess a number quick !!");
    println!("Now quickly enter what you have guessed !");
    
    let mut myguess = String::new();

    io::stdin()
	.read_line(&mut myguess)
	.ok()
	.expect("can not read !! I am blind again !!");
    
    println!("You guessed: {}", myguess);
}
