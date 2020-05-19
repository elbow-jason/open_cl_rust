use crate::{Dims, Output};

use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum WorkError {
    #[error("Work does not allow a zero value for any of its specified dimenions")]
    DimLengthCannotBeZero,

    #[error("Work dimensions must be 1, 2, or 3.")]
    InvalidDimsCount,

    #[error("Work size dimensions cannot have any zero values")]
    InvalidWorkSize,
}

/// WorkSize is the general, non-zero sized 3D array.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NonZeroVolume3DArray([usize; 3]);

impl NonZeroVolume3DArray {
    fn as_ptr(&self) -> *const usize {
        self.0.as_ptr()
    }
}

impl NonZeroVolume3DArray {
    fn from_dims(d: &Dims) -> Output<NonZeroVolume3DArray> {
        let work_size: [usize; 3] = match d {
            Dims::One(x) => [*x, 1, 1],
            Dims::Two(x, y) => [*x, *y, 1],
            Dims::Three(x, y, z) => [*x, *y, *z],
        };
        match work_size {
            [0, _, _] | [_, 0, _] | [_, _, 0] => Err(WorkError::InvalidWorkSize)?,
            w => Ok(NonZeroVolume3DArray(w)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GlobalWorkSize(NonZeroVolume3DArray);

impl GlobalWorkSize {
    pub fn from_dims(dims: &Dims) -> Output<GlobalWorkSize> {
        let arr = NonZeroVolume3DArray::from_dims(dims)?;
        Ok(GlobalWorkSize(arr))
    }

    pub fn as_ptr(&self) -> *const usize {
        self.0.as_ptr()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LocalWorkSize {
    WorkSize(NonZeroVolume3DArray),
    Null,
}

impl LocalWorkSize {
    pub fn from_dims(dims: &Dims) -> Output<LocalWorkSize> {
        let arr = NonZeroVolume3DArray::from_dims(dims)?;
        Ok(LocalWorkSize::WorkSize(arr))
    }

    pub fn as_ptr(&self) -> *const usize {
        match self {
            LocalWorkSize::WorkSize(arr) => arr.as_ptr(),
            LocalWorkSize::Null => std::ptr::null::<usize>(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GlobalWorkOffset {
    Offset([usize; 3]),
    Null,
}

impl GlobalWorkOffset {
    pub fn from_dims(d: &Dims) -> GlobalWorkOffset {
        let work_offset: [usize; 3] = match d {
            Dims::One(x) => [*x, 0, 0],
            Dims::Two(x, y) => [*x, *y, 0],
            Dims::Three(x, y, z) => [*x, *y, *z],
        };
        GlobalWorkOffset::Offset(work_offset)
    }

    pub fn as_ptr(&self) -> *const usize {
        match self {
            GlobalWorkOffset::Offset(arr) => arr.as_ptr(),
            GlobalWorkOffset::Null => std::ptr::null::<usize>(),
        }
    }
}

/// Work is a representation of 1, 2, or 3 dimensions.
///
/// For global_size none of the specified dimensions can be 0.
///
/// For global_offset any specficied dimension can be any usize.
///
/// For local_size none of the specified dimensions can be 0.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Work {
    pub global_size: Dims,
    pub global_offset: Option<Dims>,
    pub local_size: Option<Dims>,
}

impl Work {
    pub fn new<D: Into<Dims>>(global_size: D) -> Work {
        Work {
            global_size: global_size.into(),
            global_offset: None,
            local_size: None,
        }
    }

    pub fn with_global_offset<D: Into<Dims>>(mut self, offset: D) -> Work {
        self.global_offset = Some(offset.into());
        self
    }

    pub fn with_local_size<D: Into<Dims>>(mut self, local_size: D) -> Work {
        self.local_size = Some(local_size.into());
        self
    }

    pub fn work_dims(&self) -> u32 {
        self.global_size.n_dimensions() as u32
    }

    /// A 3D array that describes the Volume of the Work.
    /// A Work's global_work_size must describe a non-zero Volume.
    /// For example, `[4, 3, 2]` is a 4 by 3 by 2 Volume that does not
    /// result in an empty volume (4 * 3 * 2 != 0); to drive this point
    /// home the Volume `[3, 3, 0]` is not a valid global_work_size because
    /// the product of its elements equal 0.
    pub fn global_work_size(&self) -> Output<GlobalWorkSize> {
        GlobalWorkSize::from_dims(&self.global_size)
    }

    /// A 3D array that describes the 3 dimensional offset of the Work.
    /// The `global_work_size` can be `None` or can be specified as a `Dims`.
    /// Because the `global_work_offset` describes an of a 3 dimensional
    /// collection/buffer the dimensionality of the data can be zero.
    /// `Some([0, 0, 0])` is a valid `global_work_offset`.
    pub fn global_work_offset(&self) -> GlobalWorkOffset {
        match &self.global_offset {
            Some(dims) => GlobalWorkOffset::from_dims(dims),
            None => GlobalWorkOffset::Null,
        }
    }

    pub fn local_work_size(&self) -> Output<LocalWorkSize> {
        match &self.local_size {
            Some(dims) => LocalWorkSize::from_dims(dims),
            None => Ok(LocalWorkSize::Null),
        }
    }

    pub fn n_items(&self) -> usize {
        self.global_size.n_items()
    }
}

impl<T> From<T> for Work
where
    T: Into<Dims>,
{
    fn from(val: T) -> Work {
        Work::new(val)
    }
}
