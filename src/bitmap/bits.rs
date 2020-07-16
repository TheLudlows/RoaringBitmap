#[inline]
pub fn split(value: u32) -> (u16, u16) {
    ((value >> 16) as u16, value as u16)
}
#[inline]
pub(crate) fn pos(index: u16) -> usize {
    index as usize / 64
}

#[inline]
pub(crate) fn mask(index: u16) -> usize {
    1 << index as usize % 64
}