pub trait Upcast<T> {
    fn upcast(&self) -> T;
}
