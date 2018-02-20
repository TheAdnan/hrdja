extern crate rand;
use rand::Rng;

#[derive(Clone)]
struct Mine{
    position: (u32, u32)
}

#[derive(Clone)]
struct MineField{
    size: (u32, u32),
    mines: Vec<Mine>
}

impl MineField{
    pub fn generate_mines(&mut self, num_of_mines: u32){
        for i in 0..num_of_mines{
            let mina = Mine{
                position: (rand::thread_rng().gen_range(self.size.0, self.size.1), rand::thread_rng().gen_range(self.size.0, self.size.1))
            };
            &self.mines.push(mina);
        }
    }

    pub fn print_field(&self){
        for i in &self.mines{
            println!("{:?}\n", i.position);
        }
    }
}


fn main() {

    let mut polje = MineField{
        size: (22, 44),
        mines: Vec::new()
    };

    &polje.generate_mines(12);

    &polje.print_field();
    
}
