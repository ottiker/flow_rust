extern crate flow;

use flow::Adapter;
use flow::sequence::{Sequence, State, Handler};
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub struct FsAdapter<T> {
	_cache: T
}

impl<K: Eq + Hash, V> Adapter<FsAdapter<HashMap<K, V>>> for FsAdapter<HashMap<K, V>> {

	fn new(size: usize) -> Box<FsAdapter<HashMap<K, V>>> {
		Box::new(FsAdapter {
			_cache: HashMap::with_capacity(size)
		})
    }

	// TODO use existing LRU cache, problem:
	// struc FsAdapter {cache: LruCache</*How to get the types?*/>}
	fn set<'a, A>(&self, key: &str, val: &'a A) -> &'a A {
        println!("Cache set, called!");
		val
    }

    fn get(&self, key: &str) {
		//self._cache.insert(1, "a");
		//Some(self._cache.get(&1));
        println!("Cache get, called!");
    }

    fn del(&self, key: &str) {
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
