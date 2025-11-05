use crate::arg_parser::ParsedArgs;

pub fn dispatch(mut parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['M']);

	for option in parsed_args.options.clone() {
		match option {
			_ => ()
		}
	}
}
