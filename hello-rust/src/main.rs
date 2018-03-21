mod structs;
extern crate rand;

use rand::Rng;
use std::io::stdin;
use structs::Mine;
use structs::MineField;


fn main() {

    let mut mine_field = MineField{
        size: (3, 3),
        mines: Vec::new()
    };

    &mine_field.generate_mines();

    &mine_field.print_fields();

    let mut x = 0;
    let mut y = 0;
    loop{

        println!("Enter coordinate x: ");

        let mut input_x = String::new();
        stdin().read_line(&mut input_x);

        let mut trimmed_x = input_x.trim();
        match trimmed_x.parse::<u32>() {
            Ok(i) => x = i,
            Err(..) => println!("this was not an integer: {}", trimmed_x),
        };

        println!("Enter coordinate y: ");

        let mut input_y = String::new();
        stdin().read_line(&mut input_y);

        let mut trimmed_y = input_y.trim();
        match trimmed_y.parse::<u32>() {
            Ok(i) => y = i,
            Err(..) => println!("this was not an integer: {}", trimmed_y),
        };

        if mine_field.find_by_coordinates(x, y) {
            println!("The game is over :(");
            break;
        }
        else {
            println!("Good job!");
        }

    }

    println!("The mines are marked with o: ");

    &mine_field.print_fields_solved();

}
