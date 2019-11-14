use super::volume::Volume;

#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Dims {
    One(usize),
    Two(usize, usize),
    Three(usize, usize, usize),
}

use Dims::*;

impl Dims {
    pub fn as_size_volume(&self) -> Volume {
        match *self {
            One(x) => [x, 1, 1],
            Two(x, y) => [x, y, 1],
            Three(x, y, z) => [x, y, z],
        }
    }
    pub fn as_offset_volume(&self) -> Volume {
        match *self {
            One(x) => [x, 0, 0],
            Two(x, y) => [x, y, 0],
            Three(x, y, z) => [x, y, z],
        }
    }

    pub fn n_items(&self) -> usize {
        match *self {
            One(x) => x,
            Two(x, y) => x * y,
            Three(x, y, z) => x * y * z,
        }
    }

    pub fn n_dimensions(&self) -> u8 {
        match *self {
            One(..) => 1,
            Two(..) => 2,
            Three(..) => 3,
        }
    }

    pub fn transpose(&self) -> Dims {
        match *self {
            One(x) => Two(1, x),
            Two(x, y) => Two(y, x),
            Three(x, y, z) => Three(x, z, y)
        }
    }
}

impl From<usize> for Dims {
    fn from(x: usize) -> Dims {
        Dims::One(x)
    }
}


impl From<(usize,)> for Dims {
    fn from((x,): (usize,)) -> Dims {
        Dims::One(x)
    }
}


impl From<(usize, usize)> for Dims {
    fn from((x, y): (usize, usize)) -> Dims {
        Dims::Two(x, y)
    }
}


impl From<(usize, usize, usize)> for Dims {
    fn from((x, y, z): (usize, usize, usize)) -> Dims {
        Dims::Three(x, y, z)
    }
}