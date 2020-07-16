use std::iter::FromIterator;

use crate::bitmap::{bits, Container, RoaringBitmap};

impl RoaringBitmap {
    pub fn new() -> RoaringBitmap {
        RoaringBitmap {
            containers: Vec::new(),
        }
    }
    pub fn insert(&mut self, x: u32) -> bool {
        let (key, index) = bits::split(x);
        let result = self.containers.binary_search_by_key(&key, |container| container.key);
        let container = match result {
            Ok(loc) => &mut self.containers[loc],
            Err(loc) => {
                self.containers.insert(loc, Container::new(key));
                &mut self.containers[loc]
            }
        };
        container.insert(index)
    }

    pub fn contains(&self, x: u32) -> bool {
        let (key, index) = bits::split(x);
        match self.containers.binary_search_by_key(&key, |container| container.key) {
            Ok(loc) => self.containers[loc].contains(index),
            Err(_) => false,
        }
    }

    pub fn remove(&mut self, x: u32) -> bool {
        let (key, index) = bits::split(x);
        match self.containers.binary_search_by_key(&key, |container| container.key) {
            Ok(loc) => self.containers[loc].remove(index),
            Err(_) => false,
        }
    }

    pub fn print(&self) {
        println!("containers count:{}", self.containers.len());
        for c in &self.containers {
            println!("container key:{},len:{},store type:{}", c.key, c.len, c.type_of())
        }
    }
}

impl FromIterator<u32> for RoaringBitmap {
    fn from_iter<I: IntoIterator<Item=u32>>(iterator: I) -> RoaringBitmap {
        let mut rb = RoaringBitmap::new();
        rb.extend(iterator);
        rb
    }
}

impl Extend<u32> for RoaringBitmap {
    fn extend<I: IntoIterator<Item=u32>>(&mut self, iterator: I) {
        for value in iterator {
            self.insert(value);
        }
    }
}