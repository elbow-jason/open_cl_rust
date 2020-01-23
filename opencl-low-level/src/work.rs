// use std::marker::PhantomData;
use std::cmp::max;
use crate::{Dims, Error, Output};

#[derive(Debug, Fail, Clone, Eq, PartialEq)]
pub enum WorkError {
    #[fail(display = "Work does not allow a zero value for any of its specified dimenions")]
    DimLengthCannotBeZero,

    #[fail(display = "Work dimensions must be 1, 2, or 3.")]
    InvalidDimsCount,

    #[fail(display = "Work size dimensions cannot have any zero values")]
    InvalidWorkSize,
}

const INVALID_DIM_LENGTH: Error = Error::WorkError(WorkError::DimLengthCannotBeZero);
const INVALID_DIMS_COUNT: Error = Error::WorkError(WorkError::InvalidDimsCount);
const INVALID_WORK_SIZE: Error = Error::WorkError(WorkError::InvalidWorkSize);


// #[repr(C)]
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct NonEmptyVolume {
//     dims: Dims,
//     _phantom: PhantomData<()>,
// }

// #[inline]
// fn cannot_be_zero(num: usize) -> Result<(), NonEmptyVolumeError> {
//     if num == 0 {
//         Err(NonEmptyVolumeError::DimCannotBeZero)
//     } else {
//         Ok(())
//     }
// }

// impl NonEmptyVolume {
//     pub fn one_dim(x: usize) -> Result<NonEmptyVolume, NonEmptyVolumeError> {
//         cannot_be_zero(x)?;
//         Ok(NonEmptyVolume {
//             dims: Dims::One(x),
//             _phantom: PhantomData,
//         })
//     }

//     pub fn two_dim(x: usize, y: usize) -> Result<NonEmptyVolume, NonEmptyVolumeError> {
//         cannot_be_zero(x)?;
//         cannot_be_zero(y)?;
//         Ok(NonEmptyVolume {
//             dims: Dims::Two(x, y),
//             _phantom: PhantomData,
//         })
//     }

//     pub fn three_dim(x: usize, y: usize, z: usize) -> Result<NonEmptyVolume, NonEmptyVolumeError> {
//         cannot_be_zero(x)?;
//         cannot_be_zero(y)?;
//         cannot_be_zero(z)?;
//         Ok(NonEmptyVolume {
//             dims: Dims::Three(x, y, z),
//             _phantom: PhantomData,
//         })
//     }

//     pub fn dims(&self) -> Dims {
//         self.dims.clone()
//     }

//     pub fn as_size_volume(&self) -> [usize; 3] {
//         self.dims.as_size_volume()
//     }
//     pub fn as_offset_volume(&self) -> [usize; 3] {
//         self.dims.as_offset_volume()
//     }

//     pub fn n_items(&self) -> usize {
//         self.dims.n_items()
//     }

//     pub fn n_dimensions(&self) -> u8 {
//         self.dims.n_dimensions()
//     }
// }

// impl From<Dims> for NonEmptyVolume {
//     fn from(dims: Dims) -> NonEmptyVolume {
//         let v_result = match dims {
//             Dims::One(x) => NonEmptyVolume::one_dim(x),
//             Dims::Two(x, y) => NonEmptyVolume::two_dim(x, y),
//             Dims::Three(x, y, z) => NonEmptyVolume::three_dim(x, y, z),
//         };
//         v_result.expect("Failed to convert from Dims to NonEmptyVolume")
//     }
// }

// impl From<usize> for NonEmptyVolume {
//     fn from(num: usize) -> NonEmptyVolume {
//         NonEmptyVolume::one_dim(num)
//             .unwrap_or_else(|e| panic!("Failed to convert usize to NonEmptyVolume {:?}", e))
//     }
// }

// impl From<(usize,)> for NonEmptyVolume {
//     fn from((x,): (usize,)) -> NonEmptyVolume {
//         NonEmptyVolume::one_dim(x).unwrap_or_else(|e| {
//             panic!(
//                 "Failed to convert (usize,) to NonEmptyVolume - {:?} - {:?}",
//                 (x,),
//                 e
//             )
//         })
//     }
// }

// impl From<(usize, usize)> for NonEmptyVolume {
//     fn from((x, y): (usize, usize)) -> NonEmptyVolume {
//         NonEmptyVolume::two_dim(x, y).unwrap_or_else(|e| {
//             panic!(
//                 "Failed to convert (usize, usize) to NonEmptyVolume - {:?} - {:?}",
//                 (x, y),
//                 e
//             )
//         })
//     }
// }

// impl From<(usize, usize, usize)> for NonEmptyVolume {
//     fn from((x, y, z): (usize, usize, usize)) -> NonEmptyVolume {
//         NonEmptyVolume::three_dim(x, y, z).unwrap_or_else(|e| {
//             panic!(
//                 "Failed to convert (usize, usize, usize) to NonEmptyVolume - {:?} - {:?}",
//                 (x, y, z),
//                 e
//             )
//         })
//     }
// }

// #[inline]
// fn check_size_array(array: &[usize; 3]) -> Output<()> {
//     match array {
//         [0, _, _] | [_, 0, _] | [_, _, 0] => Err(INVALID_WORK_SIZE),
//         _ => Ok(())
//     }
// }

// #[inline]
// fn check_work_dims(wd: u32) -> Output<()> {
//     match wd {
//         0 => Err(INVALID_WORK_DIMS),
//         x if x > 3 => Err(INVALID_WORK_DIMS),
//         _ => Ok(()),
//     }
// }

// pub struct ClWork {
//     pub work_dims: u32,
//     pub global_work_size: [usize; 3],
//     pub global_work_offset: Option<[usize; 3]>,
//     pub local_work_size: Option<[usize; 3]>
// }

// impl ClWork {
//     pub fn check(&self) -> Output<()> {
//         check_work_dims(self.work_dims)?;
//         check_size_array(&self.global_work_size)?;
//         match &self.local_work_size {
//             Some(array) => check_size_array(array),
//             None => Ok(()),
//         }
//     }
// }

// impl From<&Work> for ClWork {
//     fn from(w: &Work) -> ClWork {
//         ClWork {
//             work_dims: w.work_dims(),
//             global_work_size: w.global_work_size(),
//             global_work_offset: w.global_work_offset(),
//             local_work_size: w.local_work_size(),
//         }
//     }
// }

/// WorkSize is the general, non-zero sized 3D array.
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
            [0, _, _] | [_, 0, _] | [_, _, 0] => Err(INVALID_WORK_SIZE),
            w => Ok(NonZeroVolume3DArray(w))
        }
    }
}

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
