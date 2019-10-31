#[doc(hidden)]
#[macro_export]
macro_rules! __enum_two_way_from {
    ($source_type:ident, $dest_type:ident, { $($source_value:ident => $dest_value:expr),* }) => {
        // use std::convert::From;
        impl From<$source_type> for $dest_type {
            fn from(source_value: $source_type) -> $dest_type {
                (source_value as $dest_type)
            }
        }

        impl From<&$source_type> for $dest_type {
            fn from(source_value: &$source_type) -> $dest_type {
                $source_type::from(*source_value) as $dest_type
            }
        }

        impl From<$dest_type> for $source_type {
            fn from(dest_value: $dest_type) -> $source_type {
                // when this low level API is a little more mature,
                // we can add a config flag to remove this check and simply
                // mem::transmute. Better off checking for now.
                // TODO: Investigate if's vs HashMap vs other KV performance.
                $(
                    if dest_value == $dest_value as $dest_type {
                        return $source_type::$source_value
                    }
                )*
                let source_str = stringify!($source_type);
                let dest_str = stringify!($right_type);

                // Note: replace this with a TryFrom some day....
                panic!("From failed for {:?} to {:?} for value {:?}", dest_str, source_str, dest_value);
            }
        }
    };

    ($source_type:ident, $dest_type:ident, $source_value:expr, $dest_value:expr) => {
        two_way_from!($source_type, $dest_type, $source_value, $dest_value)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __enum_define {
    ($name:ident, { $($field:ident => $value:expr),* }) => {
        #[allow(non_camel_case_types)]

        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
        pub enum $name {
            $(
                $field = $value as isize,
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __test_enum_converter {
    ($enum_type:ident, $other_type:ty ,{ $($enum_value:expr => $other_value:expr),* }) => {
        paste::item! {
            $(
                #[allow(non_snake_case)]
                #[test]
                fn [<type_ $enum_type $enum_value _converts_to_and_from_ $other_type>]() {
                    assert_eq!($enum_type::from($other_value), $enum_type::$enum_value);
                    assert_eq!($other_type::from($enum_type::$enum_value), $other_value);
                }
            )*
        }
    };
}
