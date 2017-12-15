extern crate rand;

use rand::Rng;
use std::io::stdin;
use rand::thread_rng;

fn main() {

	let bolt_sign = '🔩';

    println!("{} Selam alejk my brozer {} \n", bolt_sign, bolt_sign);
    
    println!("You can reply here: ");

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).expect("An error ocurred");

    println!("\nYou replied: {}", input_str);

    generate_secret_number();

    println!("\Random generated tuple: {:?}", generate_tuple_random());
}

fn generate_secret_number(){
	let number = thread_rng().gen_range(1,33);
	println!("Secret number is: {}", number);
}

fn generate_tuple_random() -> (u32, u32, u32){
	let tup = (thread_rng().gen_range(1,33), thread_rng().gen_range(1,33), thread_rng().gen_range(1,33));
	return tup
}
