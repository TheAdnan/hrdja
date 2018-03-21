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

    pub fn find_by_coordinates(&mut self, x: u32, y: u32) -> bool{
        for element in self.mines.iter(){
            if element.position.0 == x && element.position.1 == y && element.active == false{
                self.remove_solved(x, y);
                return false
            }
        }
        true
    }


    pub fn generate_mines(&mut self){

        &self.generate_empty_fields();

        for element in self.mines.iter_mut(){
            if 1 == rand::thread_rng().gen_range(0, 3){
                (*element).active = true;
            }
        }

        &self.mines.reverse();

    }

    fn generate_empty_fields(&mut self){
        let mut x = self.size.0;
        let mut y = self.size.1;
        while x > 0{
            while y > 0{
                let mina = Mine{
                    position: (x - 1, y - 1),
                    active: false
                };
                &self.mines.push(mina);
                y = y - 1;
            }
            y = self.size.1;
            x = x - 1;
        }

    }

    pub fn print_fields_solved(&self){
        let y = &self.size.1;
        for i in self.mines.iter(){
            if (y - 1) == i.position.1{
                println!("");
            }
            if i.active {
                print!("o")
            }
            else{
                print!("x");
            }
        }
        println!("");
    }

    pub fn print_fields(&self){

        let y = self.size.1;
        for i in self.mines.iter(){
            print!("x");
            if (y - 1) == i.position.1{
                println!("");
            }

        }
        println!("");
    }

    fn remove_solved(&self, a: u32, b: u32){
        let y = &self.size.1;
        for i in self.mines.iter(){
            if (y - 1) == i.position.1{
                println!("x");
            }
            else if a == i.position.0 && b == i.position.1{
                print!("-")
            }
            else{
                print!("x");
            }

        }
        println!("");
    }
}