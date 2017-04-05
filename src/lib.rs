extern crate flow;
extern crate lru_cache;

use lru_cache::LruCache;
use flow::Adapter;
use flow::sequence::{Sequence, State, Handler};
use std::hash::Hash;
use std::cmp::Eq;

pub struct FsAdapter<K: Eq + Hash, V> {
	_cache: LruCache<K, V>
}

impl<K: Eq + Hash, V> FsAdapter<K, V> {
    pub fn new(lru_size: usize) -> Box<FsAdapter<K, V>> {
		Box::new(FsAdapter {
            _cache: LruCache::new(lru_size)
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
