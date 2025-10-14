use std::process::exit;

mod arg_parser;
mod requests;
mod operations;

fn main() {
	let parsed_args = arg_parser::parse();

	match parsed_args.operation {
		'S' => operations::sync::dispatch(parsed_args),
		other => {
			eprintln!("error: invalid operation \'{}\'", other);
			exit(1);
		},
	}
}
