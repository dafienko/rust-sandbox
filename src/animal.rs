
#[derive(Debug)]
pub struct AnimalInstance {
	name: String,
	age: i32
}

impl AnimalInstance {
	pub fn getName(&self) -> &str {
		self.name.as_str()
	}
}

pub trait Animal {
	fn speak();
	fn getAnimalInstance() -> AnimalInstance;
}


pub struct Cow {

}

pub struct Dog {

}