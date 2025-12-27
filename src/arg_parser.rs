use std::env;
use std::process::exit;
use std::io::{stdin, Read};

#[derive(Debug)]
pub struct ParsedArgs {
	pub operation: Option<char>,
	pub options: Vec<char>,
	pub args: Vec<String>,
}

impl ParsedArgs {
	pub fn option(&self, option: char) -> bool {
		self.options.contains(&option)
	}

	pub fn check_validity(&self, allowed_options: &[char]) {
		for option in self.options.iter() {
			if !allowed_options.contains(option) {
				eprintln!("fatal: invalid option: \'{}\'", option);
				exit(1);
			}
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

	// used to allow only one -
	let mut allow_reading_from_stdin = true;

	for arg in argv {
		// if flag parsing is disabled treat everything as an arg
		if !parse_flags {
			args.push(arg);
		}

		else if arg == "-" && allow_reading_from_stdin {
			allow_reading_from_stdin = true;

			args.append(&mut read_from_stdin());
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
						eprintln!("fatal: only one operation may be used at a time");
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

	ParsedArgs {
		operation: operation_opt,
		options: options,
		args: args,
	}
}

fn read_from_stdin() -> Vec<String> {
	let mut buf = String::new();
	let result = stdin().read_to_string(&mut buf);

	if result.is_err() {
		eprintln!("error: couldn't read from stdin");
		exit(1);
	}

	buf
		.split('\n')
		.map(|s| s.to_string())
		.collect()
}
