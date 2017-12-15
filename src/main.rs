extern crate rand;

use rand::Rng;
use std::io::stdin;
use rand::thread_rng;

fn main() {
    println!("Selam alejk my brozer\n");
    
    println!("You can reply here: ");

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).expect("An error ocurred");

    println!("\nYou replied: '{}'", input_str);

    generate_secret_number();
}

fn generate_secret_number(){
	let number = thread_rng().gen_range(1,33);
	println!("Secret number is: {}", number);
}
