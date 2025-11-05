use crate::arg_parser::ParsedArgs;

mod info;
mod search;

pub fn dispatch(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S', 'h', 's', 'i', 'a']);

	for option in parsed_args.options.clone() {
		match option {
			'i' => { info::command_info(parsed_args); break; },
			's' => { search::command_search(parsed_args); break;},
			'h' => todo!("print help"),
			_ => ()
		}
	}
}
