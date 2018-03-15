mod structs;
extern crate rand;

use rand::Rng;
use structs::Mine;
use structs::MineField;


fn main() {

    let mut polje = MineField{
        size: (3, 3),
        mines: Vec::new()
    };

    &polje.generate_mines(12);

    &polje.print_fields();

}
