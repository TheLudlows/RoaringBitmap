extern crate Roaring;

use std::iter::FromIterator;

use Roaring::RoaringBitmap;

#[test]
fn test() {
    let mut b = RoaringBitmap::new();
    assert!(b.insert(1));
    assert!(!b.insert(1));
    assert!(b.contains(1));
    println!("{}",b.contains(1));

    let mut b1 = RoaringBitmap::from_iter(1..2000);
    assert!(b1.contains(1000));
    assert!(!b1.contains(2000));
    b1.remove(1000);
    assert!(!b1.contains(1000));

}
#[test]
fn bitmap() {
    let b2 = RoaringBitmap::from_iter(0..8000);
    b2.print()
}