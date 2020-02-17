#[macro_export]
macro_rules! panic_once {
    ($fmt:expr, $($arg:tt)+) => {
        if !std::thread::panicking() {
            panic!($fmt, $($arg)+);
        }
    }
}

#[macro_export]
macro_rules! __release_retain {
    ($snake:ident, $pascal:ident) => {
        paste::item! {

            use $crate::ffi::{
                [<clRelease $pascal>],
                [<clRetain $pascal>],
            };


            /// This function is used to "Drop" the associated OpenCL ARC object.
            ///
            /// # Safety
            /// This function atomically decrements the OpenCL reference count. Mismanagement
            /// of an object's OpenCL ARC can lead to undefined behavior.
            pub unsafe fn [<cl_release_ $snake>](cl_obj: [<cl_ $snake>]) -> Output<()> {
                let err_code = [<clRelease $pascal>](cl_obj);
                use $crate::build_output;
                build_output((), err_code)
            }

            /// This function is used to increase the atomic reference count of the associated
            /// OpenCL ARC object. This function should only be used when the OpenCL interface
            /// returns a ARC object that is not reference counted by OpenCL (yes, OpenCL let's you do that...)
            ///
            /// # Safety
            /// This function atomically decrements the OpenCL reference count. Mismanagement
            /// of an object's OpenCL ARC can lead to undefined behavior.
            pub unsafe fn [<cl_retain_ $snake>](cl_obj: [<cl_ $snake>]) -> Output<()> {
                let err_code = [<clRetain $pascal>](cl_obj);
                use $crate::build_output;
                build_output((), err_code)
            }
        }
    };
}
