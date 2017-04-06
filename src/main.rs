extern crate flow;
extern crate flow_rust;
extern crate lru;

use std::env;
use std::fs::canonicalize;
use flow::{Flow, Adapter};
use flow_rust::FsAdapter;
use std::collections::HashMap;

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
	let adapter: Box<FsAdapter<HashMap<&str, isize>>> = FsAdapter::new(2);
	Flow(adapter)(&sequence_id, &default_role);
}
