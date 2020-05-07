// //! OpenCL vector types.
// //!
// //! All operations use wrapped arithmetic and will not overflow.
// //!
// //! Some of these macros have been adapted (shamelessly copied) from those in
// //! the ocl crate which adapted (shamelessly copied) them from the standard library.
// //!
// //! See crate level documentation for more information.
// //!
// //! [TODO]: Add scalar-widening operations (allowing vec * scl for example).
// //! [TODO]: impl Hash.

// // #![allow(unused_imports)]

// use std::fmt::{Display, Formatter, Result as FmtResult};
// use std::ops::*;
// use std::iter::{Sum, Product};
// use std::hash::{Hash, Hasher};
// use std::cmp::{Eq, Ord, Ordering};
// use num_traits::{Zero, One};

// use super::{NumberNewType, };

// macro_rules! expand_val {
//     ($( $junk:expr, $val:expr ),+) => ( $($val),+ );
// }



// macro_rules! __define_vec {
//   ($new_type:ident, $t:ty, $inner_len:expr, $outer_len:expr) => {
//     #[derive(Debug, Clone, Copy, Default)]
//     #[repr(transparent)]
//     pub struct $new_type([$t; $len])

//     impl Number for $new_type {
//       type T = $t;
//       type Inner = [$t; $inner_len];
//       type Outer = [$t; $outer_len];
      
//       fn new(val: Self::Outer) -> Result<Self>;
//       fn inner_len() -> usize;
//       fn outer_len() -> usize;
//       fn into_inner(self) -> Self::Inner;
//       fn inner_ref(&self) -> &Self::Inner;
//       fn inner_mut_ref(&mut self) -> &mut Self::Inner;
//       fn inner_slice(&self) -> &[Self::Inner];
//       fn inner_slice_mut(&self) -> &mut [Self::Inner];
    
//       fn into_outer(self) -> Self::Outer;
//       fn from_bytes(bytes: &[u8]) -> Self;
//     }
//   }
// }

