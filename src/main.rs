extern crate rand;

use std::io::stdin;
use std::thread;

mod functions;
mod user;

fn main() {

    // Structs

    let user_thread = thread::spawn(||{
        let user = user::User::make(1, String::from("Sakib"), String::from("sakib@sakib.co.uk"));

        println!("{:?}", user.username);

        println!("{:?}", user.get_id());
    });

    thread::sleep(std::time::Duration::from_millis(100));

    let strings_thread = thread::spawn(||{
        //String manipulation

        let bolt_sign = '🔩';

        let some_string = String::from("This is some string which length is: ");

        println!("{:?} {:?}", some_string, functions::calculate_string_length(&some_string));

        println!("{}  Selam alejk my brozer {} \n", bolt_sign, bolt_sign);

        println!("You can reply here: ");

        let mut input_str = String::new();

        stdin().read_line(&mut input_str).expect("An error ocurred");

        println!("\nYou replied: {}", input_str);
    });

    let random_thread = thread::spawn(||{
        let random_number: u32 = functions::generate_secret_number();
        if random_number > 10 {
            println!("The number is bigger than 10");
        }
            else {
                println!("The number is smaller than 10");
            }

        println!("\nRandom generated tuple: {:?}", functions::generate_tuple_random());
    });

    random_thread.join().unwrap();
    user_thread.join().unwrap();
    strings_thread.join().unwrap();

}