#[derive(Debug)]
pub struct ResultTracker<'a> {
	failed: Vec<&'a str>,
}

impl<'a> ResultTracker<'a> {
	pub fn new() -> ResultTracker<'a> {
		ResultTracker {
			failed: Vec::new(),
		}
	}

	pub fn register<T, E>(&mut self, id: &'a str, result: &Result<T, E>) {
		if result.is_err() {
			self.failed.push(id);
		}
	}

	pub fn summarize(&self) {
		eprint!("\n");

		let n_failed = self.failed.len();

		if n_failed == 0 {
			eprintln!("All successful");
		} else {
			eprintln!("warning: {n_failed} fail(s) for identifiers:");
			eprintln!("{}", self.failed.join(", "));
		}
	}
}
