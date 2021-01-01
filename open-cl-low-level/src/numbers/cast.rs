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

impl<T, U> NumCastFrom<Vec<T>> for Vec<U>
where
    U: NumCastFrom<T>,
{
    fn num_cast_from(val: Vec<T>) -> Option<Vec<U>> {
        let mut output: Vec<U> = Vec::with_capacity(val.len());
        for item in val.into_iter() {
            match NumCastFrom::num_cast_from(item) {
                Some(casted) => output.push(casted),
                None => return None,
            }
        }
        Some(output)
    }
}
