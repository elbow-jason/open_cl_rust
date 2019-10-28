// #[doc(hidden)]
// #[macro_export]
// macro_rules! __two_way_from_impl {
//     ($source_type:ident, $dest_type:ident, { $($source_value:expr => $_dest_value:ident = $cl_value:expr),* }) => {
//         use std::convert::From;
//         paste::item! {
//             impl From<$source_type> for $dest_type {
//                 fn from(source_value: $source_type) -> $dest_type {
//                     $(
//                         if source_value == $source_type::$source_value as $source_type {
//                             return $cl_value
//                         }
//                     )*
//                     let source_str = stringify!($source_type);
//                     let dest_str = stringify!($right_type);
//                     panic!("From failed for {:?} to {:?} for value {:?}", source_str, dest_str, source_value);
//                 }
//             }

//             impl From<&$source_type> for $dest_type {
//                 fn from(source_value: &$source_type) -> $dest_type {
//                     $(
//                         if *source_value == $source_type::$source_value as $source_type {
//                             return $cl_value
//                         }
//                     )*
//                     let source_str = stringify!($source_type);
//                     let dest_str = stringify!($right_type);
//                     panic!("From failed for {:?} to {:?} for value {:?}", source_str, dest_str, source_value);
//                 }
//             }

//             impl From<$dest_type> for $source_type {
//                 fn from(dest_value: $dest_type) -> $source_type {
//                     $(
//                         if dest_value == $cl_value as $ dest_type {
//                             return $source_type::$source_value
//                         }
//                     )*
//                     let source_str = stringify!($source_type);
//                     let dest_str = stringify!($right_type);
//                     panic!("From failed for {:?} to {:?} for value {:?}", dest_str, source_str, dest_value);
//                 }
//             }
//         }
//     };

//     ($source_type:ident, $dest_type:ident, { $($source_value:expr => $dest_value:ident),* }) => {
//         use std::convert::From;
//         paste::item! {
//             impl From<$source_type> for $dest_type {
//                 fn from(source_value: $source_type) -> $dest_type {
//                     $(
//                         if source_value == $source_type::$source_value as $source_type {
//                             return $dest_value
//                         }
//                     )*
//                     let source_str = stringify!($source_type);
//                     let dest_str = stringify!($right_type);
//                     panic!("From failed for {:?} to {:?} for value {:?}", source_str, dest_str, source_value);
//                 }
//             }

//             impl From<&$source_type> for $dest_type {
//                 fn from(source_value: &$source_type) -> $dest_type {
//                     $(
//                         if *source_value == $source_type::$source_value as $source_type {
//                             return $dest_value
//                         }
//                     )*
//                     let source_str = stringify!($source_type);
//                     let dest_str = stringify!($right_type);
//                     panic!("From failed for {:?} to {:?} for value {:?}", source_str, dest_str, source_value);
//                 }
//             }

//             impl From<$dest_type> for $source_type {
//                 fn from(dest_value: $dest_type) -> $source_type {
//                     $(
//                         if dest_value == $dest_value as $ dest_type {
//                             return $source_type::$source_value
//                         }
//                     )*
//                     let source_str = stringify!($source_type);
//                     let dest_str = stringify!($right_type);
//                     panic!("From failed for {:?} to {:?} for value {:?}", dest_str, source_str, dest_value);
//                 }
//             }
//         }
//     };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! __two_way_from_impl2 {
//     ($source_type:ident, $dest_type:ident, { $($source_value:expr => $dest_value:ident),* }) => {
//         use std::convert::From;
//         paste::item! {
//             impl From<$source_type> for $dest_type {
//                 fn from(source_value: $source_type) -> $dest_type {
//                     $(
//                         if source_value == $source_type::$source_value as $source_type {
//                             $dest_value
//                         } else
//                     )*
//                     {
//                         let source_str = stringify!($source_type);
//                         let dest_str = stringify!($right_type);
//                         panic!("From failed for {:?} to {:?} for value {:?}", source_str, dest_str, source_value)
//                     }
//                 }
//             }

//             impl From<&$source_type> for $dest_type {
//                 fn from(source_value: &$source_type) -> $dest_type {
//                     $(
//                         if *source_value == $source_type::$source_value as $source_type {
//                             $dest_value
//                         } else
//                     )*
//                     {
//                         let source_str = stringify!($source_type);
//                         let dest_str = stringify!($right_type);
//                         panic!("From failed for {:?} to {:?} for value {:?}", source_str, dest_str, source_value)
//                     }
//                 }
//             }

//             impl From<$dest_type> for $source_type {
//                 fn from(dest_value: $dest_type) -> $source_type {
//                     $(
//                         if dest_value == $dest_value as $ dest_type {
//                             $source_type::$source_value
//                         } else
//                     )*
//                     {
//                         let source_str = stringify!($source_type);
//                         let dest_str = stringify!($right_type);
//                         panic!("From failed for {:?} to {:?} for value {:?}", dest_str, source_str, dest_value)
//                     }
//                 }
//             }
//         }
//     }
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! __generate_cl_type_enum {
//     ($name:ident, { $($field:ident => $_cl_equiv:ident),* }) => {
//         #[derive(PartialEq, Debug)]
//         pub enum $name {
//             $(
//                 $field,
//             )*
//         }
//     };

//     ($name:ident, { $($field:ident => $_const_ident:ident = $_cl_value:expr),* }) => {
//         #[derive(PartialEq, Debug)]
//         pub enum $name {
//             $(
//                 $field,
//             )*
//         }
//     };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! __cl_type_enum_can_convert_to_and_from {

// ($enum_type:ident, $cl_type:ty ,{ $($enum_value:expr => $const_ident:ident = $cl_value:expr),* }) => {
//         use paste;
//         paste::item! {
//             $(
//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<type_ $enum_type $enum_value _converts_to_and_from_ $cl_type>]() {
//                     assert_eq!($enum_type::from($cl_value), $enum_type::$enum_value);
//                     assert_eq!($cl_type::from($enum_type::$enum_value), $cl_value);
//                 }
//             )*
//         }
//     };

//     ($enum_type:ident, $cl_type:ty ,{ $($enum_value:expr => $cl_value:expr),* }) => {
//         use paste;
//         paste::item! {
//             $(
//                 #[allow(non_snake_case)]
//                 #[test]
//                 fn [<type_ $enum_type $enum_value _converts_to_and_from_ $cl_value>]() {
//                     assert_eq!($enum_type::from($cl_value), $enum_type::$enum_value);
//                     assert_eq!($cl_type::from($enum_type::$enum_value), $cl_value);
//                 }
//             )*
//         }
//     };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! __generate_pub_consts {
//     ($cl_type:ty, { $($_field:ident => $const_ident:ident),* }) => {};
//     ($cl_type:ty, { $($_field:ident => $const_ident:ident = $cl_value:expr),* }) => {
//         $(
//             pub const $const_ident: $cl_type = $cl_value;
//         )*
//     };

//     () => {};
// }
