use std::mem::ManuallyDrop;

// /// Returns a Vec with *actual* length.
// pub fn vec_filled_with<T: Clone>(filler: T, len: usize) -> Vec<T> {
//     let mut out = Vec::with_capacity(len);
//     out.resize(len, filler);
//     out
// }

#[allow(dead_code)]
pub unsafe fn take_manually_drop<T>(slot: &mut ManuallyDrop<T>) -> T {
    ManuallyDrop::into_inner(std::ptr::read(slot))
}
