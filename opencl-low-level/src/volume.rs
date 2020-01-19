use libc::size_t;

pub struct Volume(usize, usize, usize);

impl Volume {
    pub fn as_ptr(&self) -> *const size_t {
        [self.0, self.1, self.2].as_ptr() as *const size_t
    }

    pub fn empty_ptr() -> *const size_t {
        std::ptr::null::<size_t>()
    }

    // pub fn option_to_ptr(option_vol: Option<Volume>) -> *const size_t {
    //     match option_vol {
    //         Some(ref vol) => vol.as_ptr(),
    //         None => std::ptr::null(),
    //     }
    // }
    // pub fn option_ref_to_ptr(option_vol: Option<&Volume>) -> *const size_t {
    //     match option_vol {
    //         Some(vol) => vol.as_ptr(),
    //         None => std::ptr::null(),
    //     }
    // }
}

impl From<[usize; 3]> for Volume {
    fn from(v: [usize; 3]) -> Volume {
        Volume(v[0], v[1], v[2])
    }
}
