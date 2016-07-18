
use std::string::String;

pub struct AppendBuffer {
	data: String
}

impl AppendBuffer {
	pub fn new() -> AppendBuffer {
		AppendBuffer {
			data: String::new(),
		}
	}
	
	pub fn append(&mut self, data: String) {
		self.data.push_str(&data);
	}
	
	pub fn get_string(&self) -> &String {
		&self.data
	}
}



