#[doc(hidden)]
#[macro_export]
macro_rules! size_t {
    ($t:ty) => {
        std::mem::size_of::<$t>() as libc::size_t
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __codes_enum {
    ($enum_name:ident, $cl_type:ident, $body:tt) => {
        $crate::__enum_define!($enum_name, $body);
        $crate::__enum_two_way_from!($enum_name, $cl_type, $body);
        $crate::__test_enum_converter!($enum_name, $cl_type, $body);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __enum_two_way_from {
    ($source_type:ident, $dest_type:ident, { $($source_value:ident => $dest_value:expr),* }) => {
        impl From<$source_type> for $dest_type {
            fn from(source_value: $source_type) -> $dest_type {
                (source_value as $dest_type)
            }
        }

        impl From<&$source_type> for $dest_type {
            fn from(source_value: &$source_type) -> $dest_type {
                $source_type::from(*source_value) as $dest_type
            }
        }

        impl From<$dest_type> for $source_type {
            fn from(dest_value: $dest_type) -> $source_type {
                // when this low level API is a little more mature,
                // we can add a config flag to remove this check and simply
                // mem::transmute. Better off checking for now.
                // TODO: Investigate if's vs HashMap vs other KV performance.
                $(
                    if dest_value == $dest_value as $dest_type {
                        return $source_type::$source_value
                    }
                )*

                // Note: replace this with a TryFrom some day....
                panic!(
                    "From failed for {:?} to {:?} for value {:?}",
                    stringify!($right_type),
                    stringify!($source_type),
                    dest_value
                );
            }
        }
    };

    ($source_type:ident, $dest_type:ident, $source_value:expr, $dest_value:expr) => {
        two_way_from!($source_type, $dest_type, $source_value, $dest_value)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __enum_define {
    ($name:ident, { $($field:ident => $value:expr),* }) => {
        #[allow(non_camel_case_types)]

        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
        pub enum $name {
            $(
                $field = $value as isize,
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __test_enum_converter {
    ($enum_type:ident, $other_type:ty ,{ $($enum_value:expr => $other_value:expr),* }) => {
        paste::item! {
            $(
                #[allow(non_snake_case)]
                #[test]
                fn [<type_ $enum_type __ $enum_value _converts_to_and_from_ $other_type>]() {
                    assert_eq!($enum_type::from($other_value), $enum_type::$enum_value);
                    assert_eq!($other_type::from($enum_type::$enum_value), $other_value);
                }
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cl_object_for_wrapper {
    ($wrapper:ident, $cl_object_type:ty, $retain_func:ident, $release_func:ident) => {
        impl $wrapper {
            #[inline]
            pub unsafe fn retain_raw_cl_object(handle: &$cl_object_type) -> Output<()> {
                $retain_func(*handle)
            }

            #[inline]
            pub unsafe fn release_raw_cl_object(handle: &$cl_object_type) -> Output<()> {
                $release_func(*handle)
            }
        }

        impl $crate::cl::ClObject<$cl_object_type> for $wrapper {
            unsafe fn raw_cl_object(&self) -> $cl_object_type {
                self.inner
            }

            unsafe fn new(cl_object: $cl_object_type) -> Output<$wrapper> {
                if cl_object.is_null() {
                    use crate::cl::ClObjectError;
                    use crate::error::Error;
                    let wrapper_name = stringify!($wrapper).to_string();
                    let e = Error::ClObjectError(ClObjectError::ClObjectCannotBeNull(wrapper_name));
                    return Err(e);
                }
                Ok($wrapper {
                    inner: cl_object,
                    _unconstructable: (),
                })
            }

            unsafe fn new_retained(cl_object: $cl_object_type) -> Output<$wrapper> {
                if cl_object.is_null() {
                    use crate::cl::ClObjectError;
                    use crate::error::Error;
                    let wrapper_name = stringify!($wrapper).to_string();
                    let e = Error::ClObjectError(ClObjectError::ClObjectCannotBeNull(wrapper_name));
                    return Err(e);
                }
                let () = $retain_func(cl_object)?;
                Ok($wrapper {
                    inner: cl_object,
                    _unconstructable: (),
                })
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_clone_for_cl_object_wrapper {
    ($wrapper:ident, $retain_func:ident) => {
        impl Clone for $wrapper {
            fn clone(&self) -> $wrapper {
                use $crate::cl::ClObject;
                unsafe {
                    $wrapper::new_retained(self.raw_cl_object()).unwrap_or_else(|e| {
                        panic!("Failed to clone {:?} due to {:?}", stringify!($wrapper), e)
                    })
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_drop_for_cl_object_wrapper {
    ($wrapper:ident, $release_func:ident) => {
        impl Drop for $wrapper {
            fn drop(&mut self) {
                use $crate::cl::ClObject;
                // println!("Dropping {:?}", self);
                unsafe {
                    $release_func(self.raw_cl_object()).unwrap_or_else(|e| {
                        panic!("Failed to drop {:?} due to {:?}", self, e);
                    })
                }
            }
        }

        impl $wrapper {
            // Decrements the reference count of the underlying cl object.
            // Incorrect usage of this function can cause a SEGFAULT.
            pub unsafe fn release_cl_object(self) -> Output<()> {
                $release_func(self.inner)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_unconstructable_cl_wrapper {
    ($wrapper:ident, $cl_object_type:ty) => {
        #[repr(C)]
        #[derive(Eq, PartialEq, Hash)]
        pub struct $wrapper {
            inner: $cl_object_type,
            _unconstructable: (),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_default_debug_for {
    ($wrapper:ident) => {
        impl std::fmt::Debug for $wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "#OpenCL::{}<[{:?}]>",
                    stringify!($wrapper),
                    self.inner
                )
            }
        }
    }
}

// all cl_release_* and cl_retain_* functions take a raw reference to the
// cl object they pertain to.
#[macro_export]
macro_rules! __release_retain {
    ($snake:ident, $pascal:ident) => {
        paste::item! {
            use crate::ffi::{
                [<clRelease $pascal>],
                [<clRetain $pascal>],
            };

            pub unsafe fn [<cl_release_ $snake>](cl_obj: [<cl_ $snake>]) -> Output<()> {
                let err_code = [<clRelease $pascal>](cl_obj);
                StatusCode::build_output(err_code, ())
            }

            pub unsafe fn [<cl_retain_ $snake>](cl_obj: [<cl_ $snake>]) -> Output<()> {
                let err_code = [<clRetain $pascal>](cl_obj);
                StatusCode::build_output(err_code, ())
            }
        }
    };
}

// #[macro_export]
// macro_rules! inspect {
//     ($item:expr) => {
//         println!("INSPECT: {}:{}:{}
//             {}: {:?}
//         ", file!(), line!(), column!(), stringify!($item), $item);
//     }
// }

#[macro_export]
macro_rules! panic_once {
    ($fmt:expr, $($arg:tt)+) => {
        if !std::thread::panicking() {
            panic!($fmt, $($arg)+);
        }
    }
}
