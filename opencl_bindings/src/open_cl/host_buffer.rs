// // NOTE: Investigate OpenCL restrictions around numbers and safety.
// use num::Num;
// use std::default::Default;

// pub struct BufferBuilder<T: Num + Clone> {
//     _len: usize,
//     _filler: T,
// }

// impl<T: Num + Clone> BufferBuilder<T> {
//     pub fn new(len: usize) -> BufferBuilder<T> {
//         BufferBuilder {
//             _len: len,
//             _filler: T::zero(),
//         }
//     }

//     pub fn with_filler(mut self, filler: T) -> BufferBuilder<T> {
//         self._filler = filler;
//         self
//     }
    
//     fn into_vec(self) -> Vec<T> {
//         let mut out = Vec::with_capacity(self._len);
//         out.resize(self._len, self._filler);
//         out
//     }
// }


// host_buffer!(ReadOnlyHostBuffer);

// macro_rules! def_mem_type {
//     ($ptr_t:ty) => {

//         impl AsPointer<$ptr_t> for $ptr_t {
//             fn as_pointer(&self) -> *const $ptr_t {
//                 self as *const $ptr_t
//             }
//         }

        
//         impl AsMutPointer<$ptr_t> for $ptr_t {
//             fn as_mut_pointer(&mut self) -> *mut $ptr_t {
//                 self as *mut $ptr_t
//             }
//         }
        
//         impl MemSize<$ptr_t> for $ptr_t {
//             fn mem_size(&self) -> usize {
//                 std::mem::size_of::<$ptr_t>()
//             }
//         }
//     }
// }

// def_mem_type!(isize);
// def_mem_type!(usize);
// def_mem_type!(u32);
// def_mem_type!(u64);
// def_mem_type!(i32);
// def_mem_type!(i64);
// def_mem_type!(f32);
// def_mem_type!(f64);

pub struct BufferOpConfig {
    pub is_blocking: bool,
    pub offset: usize,
}

impl Default for BufferOpConfig {
    fn default() -> BufferOpConfig {
        BufferOpConfig {
            is_blocking: true,
            offset: 0
        }
    }
}
