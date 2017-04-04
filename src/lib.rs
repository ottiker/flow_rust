extern crate flow;

use flow::Adapter;
use flow::sequence::{Sequence, State, Handler};

pub struct FsAdapter {
	_lru_size: usize
}

impl FsAdapter {
    pub fn new(lru_size: usize) -> Box<FsAdapter> {
        println!("New adapter called, {}", lru_size);
		Box::new(FsAdapter {
            _lru_size: lru_size
        })
    }
}

impl Adapter<FsAdapter> for FsAdapter {

	// TODO use existing LRU cache, problem:
	// struc FsAdapter {cache: LruCache</*How to get the types?*/>}
	fn set<'a, A>(key: &str, val: &'a A) -> &'a A {
        println!("Cache set, called!");
		val
    }
    fn get(&self, key: &str) {
        println!("Cache get, called!");
    }
    fn del(key: &str) {
        println!("Cache del, called!");
    }

	// TODO get sequence info from FS and SAFE
	fn seq(sequence_id: &str) {
        println!("FsAdapter seq, called!");
    }	

	// TODO how to get handlers dynamically from the fs or a network?
	// pre-compiled? the registry should handle such things...
	fn fnc(function_id: &str) {
        println!("FsAdapter fnc, called!");
    }
}
