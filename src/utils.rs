pub trait FlagsVec {
	fn exhaust_option(&mut self, option: char) -> bool;
}

impl FlagsVec for Vec<char> {
	fn exhaust_option(&mut self, option: char) -> bool {
		match self.iter().position(|f| *f == option) {
			Some(index) => {
				self.remove(index);
				true
			},
			None => false
		}
	}
}
