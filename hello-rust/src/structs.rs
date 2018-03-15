extern crate rand;

use rand::Rng;

#[derive(Clone)]
pub struct Mine{
    pub position: (u32, u32),
    pub active: bool
}

#[derive(Clone)]
pub struct MineField{
    pub size: (u32, u32),
    pub mines: Vec<Mine>
}

impl MineField{
    pub fn generate_mines(&mut self, num_of_mines: u32){
        for i in 0..num_of_mines{
            let mina = Mine{
                position: (rand::thread_rng().gen_range(self.size.0, self.size.1), rand::thread_rng().gen_range(self.size.0, self.size.1)),
                active: true
            };
            &self.mines.push(mina);
        }
        &self.generate_empty_fields();
    }

    fn generate_empty_fields(&mut self){
        for i in (&self.mines).iter(){

        }
    }

    pub fn print_fields(&self){
        let size = &self.size.1 - 1;
        let mut counter: u32 = 0;
        for i in &self.mines{
            if size % counter == 0{
                println!("\n");
            }
            println!("{:?} ", i.position);
            counter = counter + 1;
        }
    }
}