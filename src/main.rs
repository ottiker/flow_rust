extern crate flow;
extern crate flow_rust;

use std::env;
use std::fs::canonicalize;
use flow::Flow;
use flow_rust::FsAdapter;

fn main() {

    let error_prefix = "Flow-rust";
	let default_role = "*";

    // get sequence id from cli
    let sequence_id = env::args().nth(1).expect("Flow-rust: Missing sequence id.");

    // get and set working dir
    let location = env::args().nth(2).unwrap_or(String::from("."));
    let base = canonicalize(&location).expect(&error_prefix);
    env::set_current_dir(&base).expect(&error_prefix);

	// init flow with an adapter and emit first sequence
	Flow(FsAdapter::new(2))(&sequence_id, &default_role);
}
