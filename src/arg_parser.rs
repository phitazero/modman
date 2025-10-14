use std::env;
use std::process::exit;

#[derive(Debug)]
pub struct ParsedArgs {
	pub operation: char,
	pub options: Vec<char>,
	pub args: Vec<String>,
}

impl ParsedArgs {
	pub fn exhaust_option(&mut self, option: char) -> bool {
		match self.options.iter().position(|f| *f == option) {
			Some(index) => {
				self.options.remove(index);
				true
			},
			None => false
		}
	}
}


pub fn parse() -> ParsedArgs {
	let mut argv = env::args();

	let mut operation_opt: Option<char> = None; 
	let mut options: Vec<char> = Vec::new();
	let mut args: Vec<String> = Vec::new();

	// skip the executable name
	argv.next();

	// is set to false after encountering --, ending flag parsing
	let mut parse_flags = true;

	for arg in argv {
		// if flag parsing is disabled treat everything as an arg
		if !parse_flags {
			args.push(arg);
		}

		else if arg == "-" {
			unimplemented!("reading from stdin will be implemented later");
		}

		else if arg == "--" {
			parse_flags = false;
		}

		else if arg.starts_with("--") {
			unimplemented!("long flags will be implemented later");
		}

		else if arg.starts_with("-") {
			let mut chars = arg.chars();

			// skip the leading - of each flag cluster
			chars.next();

			for flag in chars {
				// capital letters are treated as operations
				// no more than 1 operation must be present
				// -h also counts as an operation (if none other are present)
				// but it's special and is handled later
				if flag.is_ascii_uppercase() {
					// throw an error if operation in already set
					if operation_opt.is_some() {
						eprintln!("error: only one operation may be used at a time");
						exit(1);
					}

					operation_opt = Some(flag);
				} else {
					options.push(flag);
				}
			}
		}

		else {
			args.push(arg);
		}
	}

	// -h is also an operation, but less prioritized
	if operation_opt.is_none() && options.contains(&'h') {
		operation_opt = Some('h');
	}

	// 1 operation must be present
	if operation_opt.is_none() {
		eprintln!("error: no operation specified");
		exit(1);
	}

	ParsedArgs {
		operation: operation_opt.unwrap(),
		options: options,
		args: args,
	}
}
