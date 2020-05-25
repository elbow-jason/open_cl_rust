macro_rules! _object_count {
    ($func:ident, $parent:ident, $flag:ident) => {{
        let mut output_size: u32 = 0;
        let err_code = $func(
            $parent.as_ptr() as *mut libc::c_void,
            $flag,
            0 as u32,
            std::ptr::null_mut(),
            &mut output_size as *mut u32,
        );
        StatusCodeError::check(err_code)?;
        Ok(output_size)
    }};
}

macro_rules! _get_objects {
    ($func:ident, $output_t:ident, $parent:ident, $flag:ident, $count:ident) => {{
        if $count == 0 {
            Ok(vec![])
        } else {
            let mut output = vec![$output_t::null_ptr(); $count as usize];
            let status = $func(
                $parent.as_ptr() as *mut libc::c_void,
                $flag,
                $count,
                output.as_mut_ptr() as *mut *mut libc::c_void,
                std::ptr::null_mut(),
            );
            StatusCodeError::check(status)?;
            Ok(output)
        }
    }};
}

// macro_rules! _info_byte_size {
//     ($func:ident, $parent:ident, $flag:ident) => {{}};
// }

// fn n_bytes_to_n_items<T: Sized>(n_bytes: usize) {
//     let size_of_t = std::mem::size_of::<T>();
//     if n_bytes % size_of_t != 0 {
//         panic!(
//             "Number of bytes was not divisible by size_of<T>: {:?} % {:?} != 0 where out where T is {}",
//             n_bytes,
//             size_of_t,
//             std::any::type_name::<T>(),
//         );
//     }
// }

macro_rules! zero_for {
    (String) => {
        0u8
    };
    (bool) => {
        0u32
    };
    (usize) => {
        0usize
    };
    (u64) => {
        0u64
    };
    (cl_device_id) => {
        $crate::cl::ClObject::null_ptr(cl_device_id)
    };

    (cl_context) => {
        $crate::cl::ClObject::null_ptr(cl_context)
    };

    (cl_context_properties) => {
        0 as cl_context_properties
    };
}

macro_rules! _empty_data {
    (One, String, $count:expr) => {
        vec![0u8; $count]
    };

    (One, bool, $n_bytes:expr) => {{
        assert_eq!($n_bytes, 4);
        zero_for!(bool)
    }};

    (One, u32, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<u32>());
        0u32
    }};

    (One, u64, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<u64>());
        0u64
    }};

    (One, usize, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<usize>());
        0usize
    }};

    (One, cl_context, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<cl_context>());
        cl_context::null_ptr()
    }};

    (One, cl_program, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<cl_program>());
        cl_program::null_ptr()
    }};

    (One, cl_device_id, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<cl_device_id>());
        cl_device_id::null_ptr()
    }};

    (One, cl_command_queue, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<cl_command_queue>());
        cl_command_queue::null_ptr()
    }};

    (One, cl_mem_flags, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<cl_mem_flags>());
        0 as cl_mem_flags
    }};

    (One, cl_command_execution_status, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<cl_command_execution_status>());
        0 as cl_command_execution_status
    }};

    (Many, usize, $n_bytes:expr) => {{
        assert_eq!($n_bytes % std::mem::size_of::<usize>(), 0);
        vec![0usize; $n_bytes / std::mem::size_of::<usize>()]
    }};

    (Many, u64, $n_bytes:expr) => {{
        assert_eq!($n_bytes % std::mem::size_of::<u64>(), 0);
        vec![0u64; $n_bytes / std::mem::size_of::<u64>()]
    }};

    (Many, u8, $n_bytes:expr) => {{
        assert_eq!($n_bytes % std::mem::size_of::<u8>(), 0);
        vec![0u8; $n_bytes / std::mem::size_of::<u8>()]
    }};

    (Many, cl_device_id, $n_bytes:expr) => {{
        assert_eq!($n_bytes % std::mem::size_of::<cl_device_id>(), 0);
        vec![cl_device_id::null_ptr(); $n_bytes / std::mem::size_of::<cl_device_id>()]
    }};

    (Many, cl_context_properties, $n_bytes:expr) => {{
        assert_eq!($n_bytes % std::mem::size_of::<cl_context_properties>(), 0);
        vec![0 as cl_context_properties; $n_bytes / std::mem::size_of::<cl_context_properties>()]
    }};
}

macro_rules! _as_ptr {
    (One, String, $data:expr) => {
        // $data is a Vec<u8> here
        $data.as_mut_ptr() as *mut libc::c_void
    };
    (Many, $t:ty, $data:expr) => {
        // $data is a Vec<usize> here
        $data.as_mut_ptr() as *mut libc::c_void
    };
    (One, $t:ident, $data:expr) => {
        // $data is a u32 here
        &mut $data as *mut _ as *mut libc::c_void
    };
}

macro_rules! _finalize_data {
    (One, String, $data:expr) => {
        $crate::cl::strings::to_utf8_string($data)
    };
    (One, bool, $data:expr) => {
        match $data {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    };
    ($one_or_many:ident, $t:ident, $data:expr) => {
        $data
    };
}

macro_rules! _output_type {
    (Many, $t:ty) => { Vec<$t> };
    (One, $t:ty) => { $t };
}

macro_rules! _data_type {
    (One, String) => { Vec<u8> };
    (One, bool) => { u32 };
    (Many, $t:ident) => { Vec<$t> };
    (One, $t:ident) => { $t };
}

#[doc(hidden)]
#[macro_export]
macro_rules! cl_get_info {
    ($one_or_many:ident, $output_t:ident, $func:ident, $parent:expr, $flag:expr) => {{
        let mut n_bytes = 0usize;
        let status_code = $func(
            $crate::cl::ClObject::as_ptr(&$parent) as *mut libc::c_void,
            $flag,
            0 as libc::size_t,
            std::ptr::null_mut(),
            &mut n_bytes,
        );
        $crate::cl::StatusCodeError::check(status_code)?;
        let mut data: _data_type!($one_or_many, $output_t) =
            _empty_data!($one_or_many, $output_t, n_bytes);
        let status = $func(
            $crate::cl::ClObject::as_ptr(&$parent) as *mut libc::c_void,
            $flag,
            n_bytes,
            _as_ptr!($one_or_many, $output_t, data),
            std::ptr::null_mut(),
        );
        $crate::cl::StatusCodeError::check(status)?;
        let out: _output_type!($one_or_many, $output_t) =
            _finalize_data!($one_or_many, $output_t, data);
        Ok(out)
    }};
    ($one_or_many:ident, $output_t:ident, $func:ident, $parent1:ident, $parent2:ident, $flag:ident) => {{
        let mut n_bytes = 0usize;
        let status_code = $func(
            $crate::cl::ClObject::as_ptr(&$parent1) as *mut libc::c_void,
            $crate::cl::ClObject::as_ptr(&$parent2) as *mut libc::c_void,
            $flag,
            0 as size_t,
            std::ptr::null_mut(),
            &mut n_bytes,
        );
        StatusCodeError::check(status_code)?;
        let mut data: _data_type!($one_or_many, $output_t) =
            _empty_data!($one_or_many, $output_t, n_bytes);
        let status = $func(
            $crate::cl::ClObject::as_ptr(&$parent1) as *mut libc::c_void,
            $crate::cl::ClObject::as_ptr(&$parent2) as *mut libc::c_void,
            $flag,
            n_bytes,
            _as_ptr!($one_or_many, $output_t, data),
            std::ptr::null_mut(),
        );
        StatusCodeError::check(status)?;
        let out: _output_type!($one_or_many, $output_t) =
            _finalize_data!($one_or_many, $output_t, data);
        Ok(out)
    }};
}
