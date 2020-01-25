pub enum VecOrSlice<'a, T: Clone> {
    Vec(Vec<T>),
    Slice(&'a [T]),
}

impl<'a, T: Clone> VecOrSlice<'a, T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            VecOrSlice::Vec(items) => items.iter(),
            VecOrSlice::Slice(items) => items.iter(),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            VecOrSlice::Vec(items) => items.as_slice(),
            VecOrSlice::Slice(items) => items,
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        match self {
            VecOrSlice::Vec(items) => items,
            VecOrSlice::Slice(items) => items.to_vec(),
        }
    }
}

impl<'a, T: Clone> From<Vec<T>> for VecOrSlice<'a, T> {
    fn from(v: Vec<T>) -> VecOrSlice<'a, T> {
        VecOrSlice::Vec(v)
    }
}

impl<'a, T: Clone> From<&'a [T]> for VecOrSlice<'a, T> {
    fn from(d: &'a [T]) -> VecOrSlice<'a, T> {
        VecOrSlice::Slice(d)
    }
}

pub enum MutVecOrSlice<'a, T: Clone> {
    Vec(Vec<T>),
    Slice(&'a mut [T]),
}

impl<'a, T: Clone> MutVecOrSlice<'a, T> {
    pub fn iter_mut(&self) -> impl Iterator<Item = &T> {
        match self {
            MutVecOrSlice::Vec(items) => items.iter(),
            MutVecOrSlice::Slice(items) => items.iter(),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            MutVecOrSlice::Vec(items) => items.as_slice(),
            MutVecOrSlice::Slice(items) => items,
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        match self {
            MutVecOrSlice::Vec(items) => items,
            MutVecOrSlice::Slice(items) => items.to_vec(),
        }
    }
}

impl<'a, T: Clone> From<Vec<T>> for MutVecOrSlice<'a, T> {
    fn from(v: Vec<T>) -> MutVecOrSlice<'a, T> {
        MutVecOrSlice::Vec(v)
    }
}

impl<'a, T: Clone> From<&'a mut [T]> for MutVecOrSlice<'a, T> {
    fn from(d: &'a mut [T]) -> MutVecOrSlice<'a, T> {
        MutVecOrSlice::Slice(d)
    }
}
