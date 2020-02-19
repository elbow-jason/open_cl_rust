#[macro_export]
macro_rules! panic_once {
    ($fmt:expr, $($arg:tt)+) => {
        if !std::thread::panicking() {
            panic!($fmt, $($arg)+);
        }
    }
}
