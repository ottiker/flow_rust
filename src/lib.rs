extern crate flow;

use flow::sequence::{Sequence, State, Handler};

//pub struct Adapter {
//	_lru_size: usize
//}

pub trait Factory {
	fn new(lru_sisce: usize) -> Box<Adapter>;
}

impl Factory for flow::Adapter {
    fn new<'a>(lru_size: usize) -> Box<Adapter> {
        println!("New adapter called, {}", lru_size);
        Box::new(Adapter {
            _lru_size: lru_size
        })
    }
}

impl flow::TAdapter for flow::Adapter {

	// TODO use existing LRU cache, problem:
	// struc Adapter {cache: LruCache</*How to get the types?*/>}
	fn set<'a, T>(key: &str, val: &'a T) -> &'a T {
        println!("Cache set, called!");
		val
    }
    fn get<'a>(key: &str) {
        println!("Cache get, called!");
    }
    fn del<'a>(key: &str) {
        println!("Cache del, called!");
    }

	// TODO get sequence info from FS and SAFE
	fn seq(sequence_id: &str) {
        println!("Adapter seq, called!");
    }	

	// TODO how to get handlers dynamically from the fs or a network?
	// pre-compiled? the registry should handle such things...
	fn fnc(function_id: &str) {
        println!("Adapter fnc, called!");
    }
}
