use std::io::stdin;

fn main() {
    println!("Selam alejk my brozer\n");
    
    println!("You can reply here: ");

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).expect("An error ocurred");

    println!("\nYou replied: '{}'", input_str);
}
