pub trait NumCastInto<T>: Sized {
    fn num_cast_into(self) -> Option<T>;
}

pub trait NumCastFrom<T>: Sized {
    fn num_cast_from(val: T) -> Option<Self>;
}

impl<T, U> NumCastInto<U> for T
where
    U: NumCastFrom<T> + Sized,
    T: Sized,
{
    fn num_cast_into(self) -> Option<U> {
        NumCastFrom::num_cast_from(self)
    }
}
