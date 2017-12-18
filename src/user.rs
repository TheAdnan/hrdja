pub struct User{
	id: u32,
	pub username: String,
	pub email: String,
	active: bool
}

impl User{
	pub fn make(id: u32, username: String, email: String) -> User{
		User {
			id: id,
			email: email,
			username: username,
			active: true
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}
}