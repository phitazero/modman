use std::env;

#[derive(Debug)]
pub struct ParsedArgs {
	pub flags: Vec<char>,
	pub args: Vec<String>,
}

impl ParsedArgs {
	fn new() -> ParsedArgs {
		ParsedArgs {
			flags: Vec::new(),
			args: Vec::new(),
		}
	}
}

pub fn parse() -> ParsedArgs {
	let mut argv = env::args();

	let mut parsed_args = ParsedArgs::new();

	// skip the executable name
	argv.next();

	// is set to false after encountering --, ending flag parsing
	let mut parse_flags = true;

	for arg in argv {
		// if flag parsing is disabled treat everything as an arg
		if !parse_flags {
			parsed_args.args.push(arg);
			continue;
		}

		if arg == "-" {
			unimplemented!("reading from stdin will be implemented later");
		}

		if arg == "--" {
			parse_flags = false;
			continue;
		}

		if arg.starts_with("--") {
			unimplemented!("long flags will be implemented later");
		}

		if arg.starts_with("-") {
			let mut chars = arg.chars();

			// skip the leading - of each flag cluster
			chars.next();

			for flag in chars {
				parsed_args.flags.push(flag);
			}

			continue;
		}

		parsed_args.args.push(arg);
	}

	parsed_args
}
