use std::mem::ManuallyDrop;

#[allow(dead_code)]
pub unsafe fn take_manually_drop<T>(slot: &mut ManuallyDrop<T>) -> T {
    ManuallyDrop::into_inner(std::ptr::read(slot))
}
