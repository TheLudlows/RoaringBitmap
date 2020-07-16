use std::fmt::{Debug, Formatter};

mod container;
mod bits;
mod bitmap;

const BITMAP_LENGTH: usize = 1024;
const ARRAY_LIMIT: u64 = 4096;

pub struct RoaringBitmap {
    pub containers: Vec<Container>
}

pub struct Container {
    key: u16,
    len: u64,
    store: Store,
}

pub enum Store {
    Array(Vec<u16>),
    Bitmap(Box<[u64; BITMAP_LENGTH]>),
    RLE,
}