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

            pub unsafe fn [<cl_release_ $snake>](cl_obj: [<cl_ $snake>]) -> Output<()> {
                let err_code = [<clRelease $pascal>](cl_obj);
                use $crate::build_output;
                build_output((), err_code)
            }

            pub unsafe fn [<cl_retain_ $snake>](cl_obj: [<cl_ $snake>]) -> Output<()> {
                let err_code = [<clRetain $pascal>](cl_obj);
                use $crate::build_output;
                build_output((), err_code)
            }
        }
    };
}
