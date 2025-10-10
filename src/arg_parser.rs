use std::env;

pub struct ParsedArgs {
	pub flags: Vec<char>,
	pub long_flags: Vec<String>,
	pub args: Vec<String>,
}

impl ParsedArgs {
	fn new() -> ParsedArgs {
		ParsedArgs {
			flags: Vec::new(),
			long_flags: Vec::new(),
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
			// trim the leading -- from --long-flag
			let arg = arg[2..].to_string();

			parsed_args.long_flags.push(arg);
			continue;
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
