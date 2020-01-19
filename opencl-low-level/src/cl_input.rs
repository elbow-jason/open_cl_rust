pub struct SizeAndPtr<T>(pub usize, pub T);

pub unsafe trait ClInput<T> where T: {
    unsafe fn size_and_ptr(&self) -> SizeAndPtr<T>;
}