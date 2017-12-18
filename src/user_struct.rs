pub struct User{
	id: u32,
	pub username: String,
	pub email: String,
	active: bool
}

pub fn init(id: u32, username: String, email: String) -> User{
	User {
		id: id,
		email: email,
		username: username,
		active: true
	}
}