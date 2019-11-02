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
                let source_str = stringify!($source_type);
                let dest_str = stringify!($right_type);

                // Note: replace this with a TryFrom some day....
                panic!("From failed for {:?} to {:?} for value {:?}", dest_str, source_str, dest_value);
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
                fn [<type_ $enum_type $enum_value _converts_to_and_from_ $other_type>]() {
                    assert_eq!($enum_type::from($other_value), $enum_type::$enum_value);
                    assert_eq!($other_type::from($enum_type::$enum_value), $other_value);
                }
            )*
        }
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __impl_clone_for_cl_object_wrapper {
    ($wrapper:ident, $retain_func:ident) => {
        impl Clone for $wrapper {
            fn clone(&self) -> $wrapper {
                unsafe {
                    let new_wrapper = $wrapper::new(self.raw_cl_object());
                    $retain_func(&new_wrapper.inner);
                    new_wrapper 
                }
            }
        }
    };

    ($wrapper:ident<T>, $retain_func:ident) => {
        impl<T: Debug> Clone for $wrapper<T> {
            fn clone(&self) -> $wrapper<T> {
                unsafe {
                    let new_wrapper = $wrapper::new(self.raw_cl_object());
                    $retain_func(&new_wrapper.inner);
                    new_wrapper 
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
                unsafe {
                    $release_func(&self.raw_cl_object());
                }
            }
        }
    };
    ($wrapper:ident<T>, $release_func:ident) => {
        impl<T: Debug> Drop for $wrapper<T> {
            fn drop(&mut self) {
                unsafe {
                    $release_func(&self.raw_cl_object());
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_cl_object_for_wrapper {
    ($wrapper:ident, $cl_object_type:ty) => {
        use crate::utils::ClObject;
        impl ClObject<$cl_object_type> for $wrapper {
            unsafe fn raw_cl_object(&self) -> $cl_object_type {
                self.inner
            }
        }
    };
    ($wrapper:ident<T>, $cl_object_type:ty) => {
        use crate::utils::ClObject;
        impl<T: Debug> ClObject<$cl_object_type> for $wrapper<T> {
            unsafe fn raw_cl_object(&self) -> $cl_object_type {
                self.inner
            }
        }
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __impl_unconstructable_cl_wrapper {
    ($wrapper:ident, $cl_object_type:ty) => {
        #[repr(C)]
        #[derive(Debug, Eq, PartialEq, Hash)]
        pub struct $wrapper {
            inner: $cl_object_type,
            _unconstructable: (),
        }

        impl $wrapper {
            pub(crate) unsafe fn new(cl_object: $cl_object_type) -> $wrapper {
                $wrapper {
                    inner: cl_object,
                    _unconstructable: (),
                }
            }
        }
    };
    ($wrapper:ident<T>, $cl_object_type:ty) => {
        #[repr(C)]
        #[cfg_attr(not(feature = "custom_debug_for_wrapper_t"), derive(Debug))]
        #[derive(Eq, PartialEq, Hash)]
        pub struct $wrapper<T> where T: Debug {
            inner: $cl_object_type,
            _phantom: PhantomData<T>,
        }

        impl<T: Debug> $wrapper<T> {
            pub(crate) unsafe fn new(cl_object: $cl_object_type) -> $wrapper<T> {
                $wrapper {
                    inner: cl_object,
                    _phantom: PhantomData
                }
            }
        }
    };
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

            pub unsafe fn [<cl_release_ $snake>](cl_obj: &[<cl_ $snake>]) {
                let status = [<clRelease $pascal>](*cl_obj);
                if let Err(e) = StatusCode::into_output(status, ()) {
                    panic!(
                        "Failed to release {} OpenCL object {:?} due to {:?}",
                        stringify!($snake),
                        cl_obj,
                        e
                    );
                }
            }

            pub unsafe fn [<cl_retain_ $snake>](cl_obj: &[<cl_ $snake>]) {
                let status = [<clRetain $pascal>](*cl_obj);
                if let Err(e) = StatusCode::into_output(status, ()) {
                    panic!(
                        "Failed to retain {} OpenCL object {:?} due to {:?}",
                        stringify!($snake),
                        cl_obj,
                        e
                    );
                }
            }
        }
    };
}


#[macro_export]
macro_rules! inspect_var {
    ($item:ident) => {
        #[cfg(not(prod))]
        println!("INSPECT: {}:{}:{}
            {}: {:?}
        ", file!(), line!(), column!(), stringify!($item), $item);
    }
}

