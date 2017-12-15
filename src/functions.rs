use rand::Rng;
use rand::thread_rng;

pub fn generate_secret_number() -> u32{
	let number = thread_rng().gen_range(1,33);
	number
}

pub fn generate_tuple_random() -> (u32, u32, u32){
	let tup = (thread_rng().gen_range(1,33), thread_rng().gen_range(1,33), thread_rng().gen_range(1,33));
	return tup
}

pub fn calculate_string_length(s: &String) -> usize{
	s.len()
}