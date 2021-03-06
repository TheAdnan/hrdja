struct User {
    name: String,
    id: u32
}

//printing function for User struct
impl User {
    fn print_me(&self){
    	println!("User id: {:?} \nUsername: {:?}", &self.id, &self.name);
    }
}

//constructor equivalent
impl User {
	fn new(name: String, id: u32) -> Self{
		Self{ name: name, id: id }
	}
}

pub trait UID{
	fn createUID(&self);
}

impl UID for User {
    fn createUID(&self){
    	println!("____Test trait____");
        &self.print_me();
        println!("____trait works____");
    }
}

fn printing_stuff<T: PartialOrd>(a: T, b: T) -> bool{
  a > b
}


mod some_modules{
    pub fn module_function(){
        println!("This is a test function");
    }
}



fn main() {
	let u = User{
		name: String::from("Ramo"),
		id: 2
	};
	let u2 = User::new(String::from("Zulfo"), 3);
	u.print_me();
	u2.print_me();
	u.createUID();
  let tr = printing_stuff(2, 5);
  println!("{:?}", tr);
	let value1: &'static str = "Selam";
	let mut value2 = value1.to_string();
	value2.push_str(" alejk");
	let value3 = value2.replace("Selam alejk", "merhaba");
	let value4 = replace_with_merhaba(&value2);
  
    some_modules::module_function();
//Arrays and vectors
  let someArray = ["This", "is", "an", "array"];

  let mut aVector: Vec<&str> = vec!["This", "is", "a", "Vector"];


  aVector.pop();
  aVector.push("also a Vector");


  for a in aVector{
    println!("{:?}", a);
  }

}


#[cfg(target_os = "linux")]
fn abs(x: i32) -> i32 {
  if x > 0 {
    x
  } else {
    -x
  }
}

#[cfg(target_os = "linux")]
fn replace_with_merhaba(s: &str) -> String {
  s.replace(s, "Merhaba")
}

#[cfg(target_os = "android")]
fn abs(x: i32){
  println!("Syyyyke");
}
