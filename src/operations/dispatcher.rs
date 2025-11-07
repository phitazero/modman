use crate::arg_parser::ParsedArgs;

fn dispatch(
	parsed_args: ParsedArgs,
	optionless_handler: fn(ParsedArgs),
	optioned_handlers: &[(char, fn(ParsedArgs))]
) {
	let possible_command_options: Vec<char> = optioned_handlers
		.iter()
		.map(|(option, _)| *option)
		.collect();

	let mut command_option: Option<char> = None;

	for option in parsed_args.options.iter() {
		if possible_command_options.contains(option) {
			command_option = Some(*option);
			break;
		}
	}

	match command_option {
		// if no command options
		None => optionless_handler(parsed_args),
		Some(command_option) => {
			for (option, handler) in optioned_handlers {
				if command_option == *option {
					handler(parsed_args);
					break;
				}
			}
		}
	}
}
