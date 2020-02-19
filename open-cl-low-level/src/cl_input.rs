// pub enum SizeAndPtr<T> {
//     Const(usize, *const T),
//     Mut(usize, *mut T),
// }

// impl<T> SizeAndPtr<T> {
//     pub fn try_as_mut_ptr(&mut self) -> Option<*mut T> {
//         match *self {
//             SizeAndPtr::Mut(_, ptr) => Some(ptr),
//             _ => None 
//         }
//     }

//     pub fn as_ptr(&self) -> *const T {
//         match *self {
//             SizeAndPtr::Mut(_, ptr) => ptr as *const T,
//             SizeAndPtr::Const(_, ptr) => ptr,
//         }
//     }

//     pub fn size(&self) -> usize {
//         match *self {
//             SizeAndPtr::Mut(size, _) |  SizeAndPtr::Mut(size, _) => *size
//         }
        
//     } 
// }

// pub unsafe trait ClInput<T>
// where
//     T: ,
// {
//     unsafe fn size_and_ptr(&self) -> SizeAndPtr<T>;
// }
