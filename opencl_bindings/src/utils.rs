
pub fn vec_filled_with<T: Clone>(filler: T, len: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(len);
    out.resize(len, filler);
    out
}
