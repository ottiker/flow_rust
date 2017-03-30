extern crate flow;
extern crate flow_rust;

use flow::flow;
use std::env;
use std::fs::canonicalize;
use flow_rust::{Adapter, Cache, Sequence, Function};

fn main() {

    let error_prefix = "Flow-rust";

    // get sequence id from cli
    let sequence_id = env::args().nth(1).expect("Flow-rust: Missing sequence id.");

    // get and set working dir
    let location = env::args().nth(2).unwrap_or(String::from("."));
    let base = canonicalize(&location).expect(&error_prefix);
    env::set_current_dir(&base).expect(&error_prefix);

    let adapter = Adapter::new(2);
    adapter.set();
    adapter.get();
    adapter.del();
    adapter.seq();
    adapter.fnc();

	let test_sequence_id = "testSequenceId".to_string();
	let test_role = "testRole".to_string();
	let e = flow(&adapter)(&test_sequence_id, &test_role);
	println!("flow event: {}, {}", e.sequence, e.role);

    println!(
        "Sequence ID: {}\nSequence Location: {:?}",
        &sequence_id, env::current_dir().unwrap()
    );
}
