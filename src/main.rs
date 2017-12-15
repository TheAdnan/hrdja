extern crate rand;

use rand::Rng;
use std::io::stdin;
use rand::thread_rng;

fn main() {

	let bolt_sign = '';

    println!("{}  Selam alejk my brozer {} \n", bolt_sign, bolt_sign);
    
    println!("You can reply here: ");

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).expect("An error ocurred");

    println!("\nYou replied: {}", input_str);

    let random_number: u32 = generate_secret_number();
    if random_number > 10 {
    	println!("The number is bigger than 10");
    }
    else {
    	println!("The number is smaller than 10");
    }

    println!("\nRandom generated tuple: {:?}", generate_tuple_random());
}

fn generate_secret_number() -> u32{
	let number = thread_rng().gen_range(1,33);
	number
}

fn generate_tuple_random() -> (u32, u32, u32){
	let tup = (thread_rng().gen_range(1,33), thread_rng().gen_range(1,33), thread_rng().gen_range(1,33));
	return tup
}
