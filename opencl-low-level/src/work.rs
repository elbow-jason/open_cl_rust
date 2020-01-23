use std::marker::PhantomData;
use crate::{Dims, Volume};

#[derive(Debug, Fail, Clone, Eq, PartialEq)]
pub enum VolumetricError {
    #[fail(display = "Volumetric does not allow a zero value for its dimenions")]
    DimCannotBeZero,
}

/// Volumetric is a representation of 1, 2, or 3 dimensions.
///
/// For Volumetric none of the dimensions can be 0. Therefore,
/// construction of a Volumetric is protected. However, a
///
/// If you need a zeroable structure see Dims.
///
#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Volumetric {
    dims: Dims,
    _phantom: PhantomData<()>,
}

#[inline]
fn cannot_be_zero(num: usize) -> Result<(), VolumetricError> {
    if num == 0 {
        Err(VolumetricError::DimCannotBeZero)
    } else {
        Ok(())
    }
}

impl Volumetric {
    pub fn one_dim(x: usize) -> Result<Volumetric, VolumetricError> {
        cannot_be_zero(x)?;
        Ok(Volumetric {
            dims: Dims::One(x),
            _phantom: PhantomData,
        })
    }

    pub fn two_dim(x: usize, y: usize) -> Result<Volumetric, VolumetricError> {
        cannot_be_zero(x)?;
        cannot_be_zero(y)?;
        Ok(Volumetric {
            dims: Dims::Two(x, y),
            _phantom: PhantomData,
        })
    }

    pub fn three_dim(x: usize, y: usize, z: usize) -> Result<Volumetric, VolumetricError> {
        cannot_be_zero(x)?;
        cannot_be_zero(y)?;
        cannot_be_zero(z)?;
        Ok(Volumetric {
            dims: Dims::Three(x, y, z),
            _phantom: PhantomData,
        })
    }

    pub fn dims(&self) -> Dims {
        self.dims.clone()
    }

    pub fn as_size_volume(&self) -> Volume {
        self.dims.as_size_volume()
    }
    pub fn as_offset_volume(&self) -> Volume {
        self.dims.as_offset_volume()
    }

    pub fn n_items(&self) -> usize {
        self.dims.n_items()
    }

    pub fn n_dimensions(&self) -> u8 {
        self.dims.n_dimensions()
    }
}

impl From<Dims> for Volumetric {
    fn from(dims: Dims) -> Volumetric {
        let v_result = match dims {
            Dims::One(x) => Volumetric::one_dim(x),
            Dims::Two(x, y) => Volumetric::two_dim(x, y),
            Dims::Three(x, y, z) => Volumetric::three_dim(x, y, z),
        };
        v_result.expect("Failed to convert from Dims to Volumetric")
    }
}

impl From<usize> for Volumetric {
    fn from(num: usize) -> Volumetric {
        Volumetric::one_dim(num)
            .unwrap_or_else(|e| panic!("Failed to convert usize to Volumetric {:?}", e))
    }
}

impl From<(usize,)> for Volumetric {
    fn from((x,): (usize,)) -> Volumetric {
        Volumetric::one_dim(x).unwrap_or_else(|e| {
            panic!(
                "Failed to convert (usize,) to Volumetric - {:?} - {:?}",
                (x,),
                e
            )
        })
    }
}

impl From<(usize, usize)> for Volumetric {
    fn from((x, y): (usize, usize)) -> Volumetric {
        Volumetric::two_dim(x, y).unwrap_or_else(|e| {
            panic!(
                "Failed to convert (usize, usize) to Volumetric - {:?} - {:?}",
                (x, y),
                e
            )
        })
    }
}

impl From<(usize, usize, usize)> for Volumetric {
    fn from((x, y, z): (usize, usize, usize)) -> Volumetric {
        Volumetric::three_dim(x, y, z).unwrap_or_else(|e| {
            panic!(
                "Failed to convert (usize, usize, usize) to Volumetric - {:?} - {:?}",
                (x, y, z),
                e
            )
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Work {
    size: Volumetric,
    offset: Option<Dims>,
    local_size: Option<Volumetric>,
}

impl Work {
    pub fn new<S>(size: S) -> Work
    where
        S: Into<Volumetric>,
    {
        Work {
            size: size.into(),
            offset: None,
            local_size: None,
        }
    }

    pub fn with_offset(mut self, offset: Dims) -> Work {
        self.offset = Some(offset);
        self
    }

    pub fn with_local_size(mut self, local_size: Volumetric) -> Work {
        self.local_size = Some(local_size);
        self
    }

    /// The number of dimensions. For OpenCL work_dim must be greater than zero
    /// and less than or equal to three.
    pub fn work_dim(&self) -> u8 {
        self.size.n_dimensions()
    }

    /// A 3D array that describes the Volume of the Work.
    /// A Work's global_work_size must describe a non-zero Volume.
    /// For example, `[4, 3, 2]` is a 4 by 3 by 2 Volume that does not
    /// result in an empty volume (4 * 3 * 2 != 0); to drive this point
    /// home the Volume `[3, 3, 0]` is not a valid global_work_size because
    /// the product of its elements equal 0.
    pub fn global_work_size(&self) -> Volume {
        self.size.as_size_volume()
    }

    /// A 3D array that describes the 3 dimensional offset of the Work.
    /// The `global_work_size` can be `None` or can be specified as a `Dims`.
    /// Because the `global_work_offset` describes an of a 3 dimensional
    /// collection/buffer the dimensionality of the data can be zero.
    /// `Some([0, 0, 0])` is a valid `global_work_offset`.
    pub fn global_work_offset(&self) -> Option<Volume> {
        self.offset.clone().map(|dims| dims.as_offset_volume())
    }

    pub fn local_work_size(&self) -> Option<Volume> {
        self.local_size.clone().map(|v| v.as_size_volume())
    }

    pub fn n_items(&self) -> usize {
        self.size.n_items()
    }
}

impl From<Volumetric> for Work {
    fn from(v: Volumetric) -> Work {
        Work::new(v)
    }
}
