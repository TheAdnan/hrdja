extern crate rand;

use std::io::stdin;

mod functions;
mod user_struct;

fn main() {

    // Structs
    let user = user_struct::init(1, String::from("Sakib"), String::from("sakib@sakib.co.uk"));

    println!("{:?}", user.username);

    //String manipulation

    let bolt_sign = '🔩';

	let some_string = String::from("This is some string which length is: ");

	println!("{:?} {:?}", some_string, functions::calculate_string_length(&some_string));

    println!("{}  Selam alejk my brozer {} \n", bolt_sign, bolt_sign);
    
    println!("You can reply here: ");

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).expect("An error ocurred");

    println!("\nYou replied: {}", input_str);

    let random_number: u32 = functions::generate_secret_number();
    if random_number > 10 {
    	println!("The number is bigger than 10");
    }
    else {
    	println!("The number is smaller than 10");
    }

    println!("\nRandom generated tuple: {:?}", functions::generate_tuple_random());
}

