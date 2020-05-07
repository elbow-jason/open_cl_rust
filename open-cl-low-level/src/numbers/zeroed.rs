
// macro_rules! __impl_zeroed_vector {
//   ($t:ty) => {
//       impl Zeroed for $t {
//           fn zeroed() -> $t {
//               zeroed_vector!($t)
//           }
//       }
//   };
// }

// macro_rules! __impl_zeroed_newtype_vector {
//   ($NewType:ty, $t:ty) => {
//       impl Zeroed for $t {
//           fn zeroed() -> $t {
//               $NewType(unsafe { std::mem::zeroed::<$t>() })
//           }
//       }
//   };
// }

// macro_rules! __impl_zeroed {
//   ($t:ty => $zero:expr) => {
//       impl Zeroed for $t {
//           fn zeroed() -> $t {
//               $zero
//           }
//       }
//   };
// }
