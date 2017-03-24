extern crate lru_time_cache;

use std::env;
use std::path::PathBuf;
use std::fs::canonicalize;
use ::lru_time_cache::LruCache;

fn main() {

    let error_prefix = "Flow-rust";

    // get sequence id from cli
    let sequence_id = env::args().nth(1).expect("Flow-rust: Missing sequence id.");

    // get and set working dir
    let location = env::args().nth(2).unwrap_or(String::from("."));
    let base = canonicalize(&location).expect(&error_prefix);
    env::set_current_dir(&base).expect(&error_prefix);

    // Cache
    let lru_cache = LruCache::<u8, String>::with_capacity(10);
    struct Cache {}
    impl Cache {
        fn get() {
            println!("Cache get called!");
        }
    }

    // Adapter
    struct Adapter {
        cache: Cache
    }
    impl Adapter {
        fn seq (file: &PathBuf) {
            println!("Sequence path: {:?}", file);
            Adapter::cache::get();
        }
    }

    Adapter::seq(&base);

    println!(
        "Sequence ID: {}\nSequence Location: {:?}",
        &sequence_id, env::current_dir().unwrap()
    );
}
