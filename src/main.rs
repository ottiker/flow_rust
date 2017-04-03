extern crate flow;
extern crate flow_rust;

use flow::Flow;
use std::env;
use std::fs::canonicalize;
use flow_rust::Adapter;
use flow_rust::Factory;

fn main() {

    let error_prefix = "Flow-rust";

    // get sequence id from cli
    let sequence_id = env::args().nth(1).expect("Flow-rust: Missing sequence id.");

    // get and set working dir
    let location = env::args().nth(2).unwrap_or(String::from("."));
    let base = canonicalize(&location).expect(&error_prefix);
    env::set_current_dir(&base).expect(&error_prefix);

	let test_sequence_id = "testSequenceId";
	let test_role = "testRole";

	let adapter = Adapter::new(2);
	//adapter.get(&adapter);
	let flow = Flow(&adapter);

	//let e = flow(&test_sequence_id, &test_role);
	//println!("flow event: {}, {}", e.sequence, e.role);

    println!(
        "Sequence ID: {}\nSequence Location: {:?}",
        &sequence_id, env::current_dir().unwrap()
    );
}
