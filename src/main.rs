extern crate flow;

use std::env;

// TODO loaders for sequence (json) and handler (Fn)

/*struct Loader {
    name: String
}

impl Loader {
    fn sequence () {},
    fn handler () {}
}*/

fn main() {

    // get sequence id from cli
    let sequence = env::args().nth(1).expect("Flow: Missing sequence id.");

    // create new flow instance and emit first sequence
    flow::new(/* pass loader*/).emit(sequence);
}
