use std::fmt;
use std::marker::PhantomData;

use crate::ffi::cl_bool;
use crate::strings;

/// ClPointer is a very short-lived struct that is used to homogenize the returns from
/// many of the CL API calls. If a call to an OpenCL C function successfully returns
/// a ClPointer that ClPointer MUST BE CONSUMED; Failure to consume the ClPointer will
/// lead to a panic when the ClPointer struct is dropped.
#[repr(C)]
pub struct ClPointer<T: Copy> {
    count: usize,
    ptr: *mut T,
    phantom: PhantomData<T>,
    is_consumed: bool,
}

/// Only u8 ClPointers can become String
impl ClPointer<u8> {
    pub unsafe fn into_string(self) -> String {
        strings::to_utf8_string(self.into_vec())
    }
}

impl From<ClPointer<u8>> for String {
    fn from(p: ClPointer<u8>) -> String {
        strings::to_utf8_string(unsafe { p.into_vec() })
    }
}

/// Special case for cl_platform_id and Platform.
// impl ClPointer<cl_platform_id> {
//     /// cl_platform_id is the only cl_wrapper that does not have a cl* function
//     /// for retaining or releaseing. cl_platform_id is part of the host memory not
//     /// the device memory and is therefore not managed by OpenCL.
//     pub unsafe fn into_many_wrappers(self) -> Output<Vec<Platform>> {
//         let mut output: Vec<Platform> = Vec::with_capacity(self.count);
//         for cl_obj in self.into_many().into_iter() {
//             match Platform::new(cl_obj) {
//                 Ok(wrapper) => output.push(wrapper),
//                 Err(e) => return Err(e),
//             }
//         }
//         Ok(output)
//     }
// }

impl From<ClPointer<cl_bool>> for bool {
    fn from(p: ClPointer<cl_bool>) -> bool {
        match unsafe { p.into_one() } {
            0 => false,
            1 => true,
            invalid_cl_bool => panic!("cl_bool was neither 0 nor 1: {:?}", invalid_cl_bool),
        }
    }
}

impl<T: Copy> ClPointer<T> {
    pub unsafe fn new(count: usize, ptr: *mut T) -> ClPointer<T> {
        ClPointer {
            count,
            ptr,
            phantom: PhantomData,
            is_consumed: false,
        }
    }

    pub unsafe fn from_vec(mut v: Vec<T>) -> ClPointer<T> {
        let ptr = v.as_mut_ptr();
        let count = v.len();
        std::mem::forget(v);
        ClPointer::new(count, ptr)
    }

    pub unsafe fn new_empty() -> ClPointer<T> {
        let mut v = vec![];
        let ptr = v.as_mut_ptr();
        std::mem::forget(v);
        ClPointer::new(0, ptr)
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    #[inline]
    pub unsafe fn into_one(self) -> T {
        self.ptr_cannot_be_null();
        self.count_must_be_one();
        let owned_ptr = self.ptr.to_owned();
        std::mem::forget(self);
        *owned_ptr
    }

    #[inline]
    pub unsafe fn into_vec(self) -> Vec<T> {
        self.ptr_cannot_be_null();
        let many = Vec::from_raw_parts(self.ptr, self.count, self.count);
        std::mem::forget(self);
        many
    }

    // #[inline]
    // pub unsafe fn into_many_retained_wrappers<W>(self) -> Output<Vec<W>>
    // where
    //     W: ClObject<T>,
    // {
    //     let mut output: Vec<W> = Vec::with_capacity(self.count);
    //     for cl_obj in self.into_many().into_iter() {
    //         match W::new_retained(cl_obj) {
    //             Ok(wrapper) => output.push(wrapper),
    //             Err(e) => return Err(e),
    //         }
    //     }
    //     Ok(output)
    // }

    #[inline]
    fn ptr_cannot_be_null(&self) {
        if self.ptr.is_null() {
            panic!("Consumed cl_pointer was a null pointer {:?}", self);
        }
    }

    #[inline]
    fn count_must_be_one(&self) {
        match self.count {
            1 => (),
            0 => {
                panic!("cl_pointer was not 1 count: {:?}", self);
            }
            _ => {
                panic!("cl_pointer was not 1 count: {:?}", self);
            }
        }
    }
}

impl<T: Copy> Drop for ClPointer<T> {
    fn drop(&mut self) {
        panic_once!("An unconsumed ClPointer was allowed to drop. This would lead to a memory leak. All ClPointers must be consumed. {:?}", self);
    }
}

impl<T: Copy> fmt::Debug for ClPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClPointer<[ptr: {:?}, count: {:?}, size_of_t: {:?}, is_consumed: {:?}]>",
            self.ptr,
            self.count,
            std::mem::size_of::<T>(),
            self.is_consumed
        )
    }
}
