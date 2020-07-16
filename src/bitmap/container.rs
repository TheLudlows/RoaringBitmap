use crate::bitmap::{Container, Store, ARRAY_LIMIT};
use crate::bitmap::bits;
use crate::bitmap::Store::{Array, Bitmap};

impl Container {
    pub fn new(key: u16) -> Container {
        let store = Array(Vec::new());
        Container {
            key,
            len: 0,
            store,
        }
    }
    pub fn insert(&mut self, index: u16) -> bool {
        if self.store.insert(index) {
            self.len += 1;
            self.auto_change();
            // need to change to bitmap or rle
            true
        } else {
            false
        }
    }

    pub fn contains(&self, index: u16) -> bool {
        self.store.contains(index)
    }

    pub fn remove(&mut self, index: u16) -> bool {
        if self.store.remove(index) {
            self.len -= 1;
            self.auto_change();
            true
        } else {
            false
        }
    }
    pub fn auto_change(&mut self) {
        match &self.store {
            Array(v) => if self.len >= ARRAY_LIMIT {
                let mut bit_store = Bitmap(Box::new([0;1024]));
                for i in v.iter() {
                    bit_store.insert(*i);
                }
                self.store = bit_store;
            },
            Bitmap(_) =>(),//do nothing one direction change
            _ => ()
        }
    }

    pub fn type_of(&self) -> String{
        match &self.store {
            Array(_) => "array".to_string(),
            Bitmap(_) => "map".to_string(),
            _ => "un know".to_string()
        }
    }
}

impl Store {
    pub fn insert(&mut self, index: u16) -> bool {
        match self {
            Array(v) => {
                v.binary_search(&index)
                    .map_err(|loc| v.insert(loc, index))
                    .is_err()
            }
            Bitmap(bits) => {
                let (pos, mask) = (bits::pos(index), bits::mask(index) as u64);
                if bits[pos] & mask == 0 {
                    bits[pos] |= mask;
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }

    pub fn contains(&self, index: u16) -> bool {
        match self {
            Array(v) => v.binary_search(&index).is_ok(),
            Bitmap(b) => b[bits::pos(index)] & (bits::mask(index) as u64) == 1,
            _ => false
        }
    }
    pub fn remove(&mut self, index: u16) -> bool {
        match self {
            Array(v) => v
                .binary_search(&index)
                .map(|loc| v.remove(loc))
                .is_ok(),
            Bitmap(b) => {
                let (pos, mask) = (bits::pos(index), bits::mask(index) as u64);
                if b[pos] & mask == 0 {
                    false
                } else {
                    b[pos] &= !mask;
                    true
                }
            }
            _ => false
        }
    }
}