use crate::arg_parser::ParsedArgs;

mod info;
mod search;

pub fn dispatch(mut parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S', 'h', 's', 'i', 'a']);

	for option in parsed_args.options.clone() {
		match option {
			'i' => info::command_info(&mut parsed_args),
			's' => search::command_search(&mut parsed_args),
			'h' => todo!("print help"),
			_ => ()
		}
	}
}
