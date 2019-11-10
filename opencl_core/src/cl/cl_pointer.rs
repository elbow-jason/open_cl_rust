use std::marker::PhantomData;
use std::fmt;

use super::cl_object::ClObject;

use crate::utils::strings;


/// ClPointer is a very short-lived struct that is used to homogenize the returns from
/// many of the CL API calls. If a call to an OpenCL C function successfully returns
/// a ClPointer that ClPointer MUST BE CONSUMED; Failure to consume the ClPointer will
/// lead to a panic when the ClPointer struct is dropped. 
#[repr(C)]
pub struct ClPointer<T> {
    count: usize,
    ptr: *mut T,
    phantom: PhantomData<T>,
}

impl ClPointer<u8> {
    pub unsafe fn into_string(self) -> String {
        strings::to_utf8_string(self.into_many())
    }
}

impl<T: Copy> ClPointer<T> {
    pub unsafe fn new(count: usize, ptr: *mut T) -> ClPointer<T> {
        ClPointer {
            count,
            ptr,
            phantom: PhantomData,
        }
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
    pub unsafe fn into_one_wrapper<W>(self) -> W where W: ClObject<T> {
        W::new(self.into_one())
    }

    #[inline]
    pub unsafe fn into_many(self) -> Vec<T> {
        self.ptr_cannot_be_null();
        let many = Vec::from_raw_parts(self.ptr, self.count, self.count);
        std::mem::forget(self);
        many
    }


    #[inline]
    pub unsafe fn into_many_wrapper<W>(self) -> Vec<W> where W: ClObject<T> {
        self.into_many()
            .into_iter()
            .map(|p| W::new(p))
            .collect()
    }

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
            },
            _ => {
                panic!("cl_pointer was not 1 count: {:?}", self);
            },
        }
    }

}

impl<T> Drop for ClPointer<T> {
    fn drop(&mut self) {
        panic!("A ClPointer was allowed to drop. This would lead to a memory leak. All ClPointers must be consumed. {:?}", self);
    }
}


impl<T> fmt::Debug for ClPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClPointer<[ptr: {:?}, count: {:?}, size_of_t: {:?}]>",
            self.ptr,
            self.count,
            std::mem::size_of::<T>()
        )
    }
}