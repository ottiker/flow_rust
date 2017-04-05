extern crate flow;

use flow::Adapter;
use flow::sequence::{Sequence, State, Handler};

pub struct FsAdapter<'a> {
	_lru_size: &'a usize
}

impl<'a> FsAdapter<'a> {
    pub fn new(lru_size: &'a usize) -> Box<FsAdapter<'a>> {
        println!("New adapter called, {}", lru_size);
		Box::new(FsAdapter {
            _lru_size: lru_size
        })
    }
}

impl<'a> Adapter<'a, FsAdapter<'a>> for FsAdapter<'a> {

	// TODO use existing LRU cache, problem:
	// struc FsAdapter {cache: LruCache</*How to get the types?*/>}
	fn set<'b, A>(key: &str, val: &'b A) -> &'b A {
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
