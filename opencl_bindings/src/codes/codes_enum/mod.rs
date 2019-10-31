pub mod helper_macros;

#[doc(hidden)]
#[macro_export]
macro_rules! __codes_enum {
    ($enum_name:ident, $cl_type:ident, $body:tt) => {
        $crate::__enum_define!($enum_name, $body);
        $crate::__enum_two_way_from!($enum_name, $cl_type, $body);
        $crate::__test_enum_converter!($enum_name, $cl_type, $body);
    };
}

    