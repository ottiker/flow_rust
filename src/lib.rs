extern crate flow;

use flow::Adapter;
//use flow::sequence::{Sequence, State, Handler};
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub fn factory<'a, T>() -> Box<FsAdapter<HashMap<&'a str, T>>> {
	Box::new(FsAdapter {
		_cache: HashMap::new()
	})
}

#[derive(Debug)]
pub struct FsAdapter<HashMap> {
	_cache: HashMap
}

impl<K: Eq + Hash + Sized, V> Adapter<FsAdapter<HashMap<K, V>>, K, V> for FsAdapter<HashMap<K, V>> {

	// TODO use existing LRU cache, problem:
	// struc FsAdapter {cache: LruCache</*How to get the types?*/>}
	fn set<'b>(&mut self, key: K, val: V) -> Option<V> {
        println!("Cache set, called!");
		//HashMap::insert(&mut self._cache, key, val)
		self._cache.insert(key, val)
    }

    fn get<'b>(&self, key: &K) {
		self._cache.get(key);
		//Some(self._cache.get(&1));
        println!("Cache get, called!");
    }

    fn del(&self, key: K) {
        println!("Cache del, called!");
    }

	// TODO get sequence info from FS and SAFE
	fn seq(sequence_id: &str) {
        println!("FsAdapter seq, called! {}", sequence_id);
    }	

	// TODO how to get handlers dynamically from the fs or a network?
	// pre-compiled? the registry should handle such things...
	fn fnc(function_id: &str) {
        println!("FsAdapter fnc, called!");
    }
}
