use crate::requests;
use crate::arg_parser::ParsedArgs;

pub fn dispatch(mut parsed_args: ParsedArgs) {
	if parsed_args.exhaust_option('h') {
		print_help();
	}
}

fn print_help() {
	todo!("help");
}
