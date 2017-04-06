extern crate flow;
extern crate lru;

use flow::Adapter;
use flow::sequence::{Sequence, State, Handler};
use lru::LruCache;
use std::hash::Hash;
use std::cmp::{Eq, PartialEq};
use std::cell::RefCell;

pub struct FsAdapter<K, V> {
	_cache: RefCell<LruCache<K, V>>
}

impl<K: Eq + Hash, V> FsAdapter<K, V> {
    pub fn new(lru_size: usize) -> Box<FsAdapter<K, V>> {
		Box::new(FsAdapter {
			_cache: RefCell::new(LruCache::new(lru_size))
		})
    }
}

impl<K: Eq + Hash, V> Adapter<FsAdapter<K, V>> for FsAdapter<K, V> {

	// TODO use existing LRU cache, problem:
	// struc FsAdapter {cache: LruCache</*How to get the types?*/>}
	fn set<'a, A>(&self, key: &str, val: &'a A) -> &'a A {
        println!("Cache set, called!");
		val
    }
    fn get(&self, key: &str) {
		let mut c = self._cache.borrow();
		c.put(1, "a");
		//c.get(1);
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
