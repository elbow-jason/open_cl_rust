// use libc::size_t;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Volume(pub usize, pub usize, pub usize);

// impl Volume {
//     pub fn sized_volume()

//     // pub fn as_ptr(&self) -> *const size_t {
//     //     [self.0, self.1, self.2].as_ptr() as *const size_t
//     // }

//     // pub fn empty_ptr() -> *const size_t {
//     //     [0, 0, 0].as_ptr() as *const size_t
//     // }

//     // pub fn option_to_ptr(option_vol: Option<Volume>) -> *const size_t {
//     //     match option_vol {
//     //         Some(ref vol) => vol.as_ptr(),
//     //         None => std::ptr::null(),
//     //     }
//     // }
//     // pub fn option_ref_to_ptr(option_vol: Option<&Volume>) -> *const size_t {
//     //     match option_vol {
//     //         Some(vol) => vol.as_ptr(),
//     //         None => std::ptr::null(),
//     //     }
//     // }
// }

// pub struct SizeVolume {
//     volume: Volume,
// }

// impl From<[usize; 3]> for Volume {
//     fn from(v: [usize; 3]) -> Volume {
//         Volume(v[0], v[1], v[2])
//     }
// }
