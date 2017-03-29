pub struct Adapter {
    _lru_size: usize
}

impl Adapter {
    pub fn new(lru_size: usize) -> Adapter {
        println!("New adapter called, {}", lru_size);
        Adapter {
            _lru_size: lru_size
        }
    }
}
pub trait Cache {
    fn set (&self);
    fn get (&self);
    fn del (&self);
}

pub trait Sequence {
    fn seq (&self);
}

pub trait Function {
    fn fnc (&self);
}

// TODO use existing LRU cache, problem:
// struc Adapter {cache: LruCache</*How to get the types?*/>}
impl Cache for Adapter {
    fn set (&self) {
        println!("Cache set, called!");
    }
    fn get (&self) {
        println!("Cache get, called!");
    }
    fn del (&self) {
        println!("Cache del, called!");
    }
}

// TODO get sequence info from FS and SAFE
impl Sequence for Adapter {
    fn seq (&self) {
        println!("Adapter seq, called!");
    }
}

// TODO how to get handlers dynamically from the fs or a network?
// pre-compiled? the registry should handle such things...
impl Function for Adapter {
    fn fnc (&self) {
        println!("Adapter fnc, called!");
    }
}
