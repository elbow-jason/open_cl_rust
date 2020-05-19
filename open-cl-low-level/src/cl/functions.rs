// // ffi object types
// use crate::cl::{cl_command_queue, cl_context, cl_device_id, cl_platform_id, cl_program, ClObject};

// // ffi data types
// use crate::cl::{
//     cl_command_queue_properties, cl_context_info, cl_device_info, cl_device_type, cl_platform_info,
//     cl_program_build_info, cl_program_info, cl_uint, ContextInfo, ProgramInfo,
// };

// use libc::{c_void, size_t};

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

// macro_rules! _run_func_for_count {
//     ($one_or_many, $output_t:ident, $func:ident, $o1:expr, $o2:expr, $flag:expr) => {{
//         let mut n_bytes = 0usize;
//         let status_code = $func(
//             $crate::cl::ClObject::as_ptr(&$o1) as *mut libc::c_void,
//             $crate::cl::ClObject::as_ptr(&$o2) as *mut libc::c_void,
//             $flag.into(),
//             0 as size_t,
//             std::ptr::null_mut(),
//             &mut n_bytes,
//         );
//     }};
// }

// macro_rules! get_empty_info {
//     ($t:ty, $o1:expr, $o2:expr, $flag:expr) => {{
//         let mut n_bytes = 0usize;
//         let status_code = $func(
//             $crate::cl::ClObject::as_ptr(&$o1) as *mut libc::c_void,
//             $crate::cl::ClObject::as_ptr(&$o2) as *mut libc::c_void,
//             $flag.into(),
//             0 as size_t,
//             std::ptr::null_mut(),
//             &mut n_bytes,
//         );
//         StatusCodeError::check(status_code)?;
//         assert!(n_bytes % std::mem::size_of<$t>() == 0);
//         let count =
//         Ok(n_bytes / std::mem::size_of<$t>())
//     }};

//     ($t:ty, $o1:expr,  $flag:expr) => {{
//         let mut n_bytes = 0usize;
//         let status_code = $func(
//             $crate::cl::ClObject::as_ptr(&$o1) as *mut libc::c_void,
//             $flag.into(),
//             0 as size_t,
//             std::ptr::null_mut(),
//             &mut n_bytes,
//         );
//         StatusCodeError::check(status_code)?;
//         assert!(n_bytes % std::mem::size_of<$t>() == 0);
//         Ok(n_bytes / std::mem::size_of<$t>())
//     }};
// }

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

//     cl_object: Obj,
//     flag: Flag,
//     func: InfoFunc5<Obj, Flag>,
// ) -> Output<size_t> {

// }

// pub unsafe fn cl_get_info5<Obj: Copy, Flag: Copy, Ret: Copy>(
//     cl_object: Obj,
//     flag: Flag,
//     func: InfoFunc5<Obj, Flag>,
// ) -> Output<ClPointer<Ret>> {
//     let num_bytes: size_t = cl_get_info_byte_count5(cl_object, flag, func)?;

//     if num_bytes == 0 {
//         return Ok(ClPointer::new_empty());
//     }

//     let mut bytes = utils::vec_filled_with(0u8, num_bytes as usize);

//     let output = bytes.as_mut_ptr() as *mut _ as *mut libc::c_void;

//     let err_code = func(cl_object, flag, num_bytes, output, std::ptr::null_mut());

//     StatusCodeError::check(err_code)?;
//     // Everything above worked so we don't want the `bytes` vec to be freed
//     // Therefore we forget it.
//     std::mem::forget(bytes);

//     let output_count = num_bytes / std::mem::size_of::<Ret>();
//     Ok(ClPointer::new(output_count, output as *mut Ret))
// }

// /// Queues an n-dimensionally ranged kernel to be executed.
// ///
// /// Blocks until the kernel is finished.
// ///
// /// # Safety
// /// Usage of an invalid ClObject is undefined behavior.
// pub unsafe fn cl_enqueue_nd_range_kernel<W: Waitlist>(
//     queue: cl_command_queue,
//     kernel: cl_kernel,
//     work: &Work,
//     waitlist: W,
// ) -> Output<cl_event> {
//     let mut tracking_event: cl_event = new_tracking_event();
//     let event_waitlist = waitlist.new_waitlist();
//     let wl = event_waitlist.as_slice();

//     let gws: GlobalWorkSize = work.global_work_size()?;
//     let lws: LocalWorkSize = work.local_work_size()?;
//     let err_code = clEnqueueNDRangeKernel(
//         queue,
//         kernel,
//         work.work_dims(),
//         work.global_work_offset().as_ptr(),
//         gws.as_ptr(),
//         lws.as_ptr(),
//         wl.waitlist_len(),
//         wl.waitlist_ptr(),
//         &mut tracking_event,
//     );

//     build_output((), err_code)?;
//     cl_finish(queue)?;

//     // TODO: Remove this check when Event checks for null pointer
//     debug_assert!(!tracking_event.is_null());
//     Ok(tracking_event)
// }

// fn new_tracking_event() -> cl_event {
//     std::ptr::null_mut() as cl_event
// }

// pub unsafe fn cl_enqueue_read_buffer<T>(
//     queue: cl_command_queue,
//     mem: cl_mem,
//     buffer: &mut [T],
//     command_queue_opts: CommandQueueOptions,
// ) -> Output<cl_event>
// where
//     T: Number,
// {
//     let mut tracking_event = new_tracking_event();
//     let waitlist = command_queue_opts.new_waitlist();
//     let wl = waitlist.as_slice();

//     // TODO: Make this a Error returning check
//     // debug_assert!(buffer.len() == device_mem.len());

//     let err_code = clEnqueueReadBuffer(
//         queue,
//         mem,
//         command_queue_opts.is_blocking as cl_bool,
//         command_queue_opts.offset,
//         buffer.buffer_byte_size(),
//         buffer.buffer_ptr(),
//         wl.waitlist_len(),
//         wl.waitlist_ptr(),
//         &mut tracking_event,
//     );
//     build_output(tracking_event, err_code)
// }

// pub unsafe fn cl_enqueue_write_buffer<T>(
//     queue: cl_command_queue,
//     mem: cl_mem,
//     buffer: &[T],
//     command_queue_opts: CommandQueueOptions,
// ) -> Output<cl_event>
// where
//     T: Number,
// {
//     let mut tracking_event = new_tracking_event();

//     let waitlist = command_queue_opts.new_waitlist();
//     let wl = waitlist.as_slice();
//     let err_code = clEnqueueWriteBuffer(
//         queue,
//         mem,
//         command_queue_opts.is_blocking as cl_bool,
//         command_queue_opts.offset,
//         buffer.buffer_byte_size(),
//         buffer.buffer_ptr(),
//         wl.waitlist_len(),
//         wl.waitlist_ptr(),
//         &mut tracking_event,
//     );

//     build_output(tracking_event, err_code)
// }

// pub unsafe fn cl_get_command_queue_info<T: Copy>(
//     command_queue: cl_command_queue,
//     flag: CommandQueueInfo,
// ) -> Output<ClPointer<T>> {
//     cl_get_info5(
//         command_queue,
//         flag as cl_command_queue_info,
//         clGetCommandQueueInfo,
//     )
// }

// get_program_info_string
// get_program_info_vec_usize
// get_program_info_bytes
// get_program_info_usize
// get_program_info_vec_device
// get_program_info_context

// pub unsafe fn cl_get_object_count<Obj, Flag, Obj2>(
//     cl_object: Obj,
//     flag: Flag,
//     func: ObjFunc<Obj, Flag, Obj2>,
// ) -> Output<u32>
// where
//     Obj: Copy,
//     Flag: Copy,
//     Obj2: Copy,
// {

// }

// pub unsafe fn cl_get_object<Obj: Copy, Flag: Copy, Obj2: Copy>(
//     cl_object: Obj,
//     flag: Flag,
//     func: ObjFunc<Obj, Flag, Obj2>,
// ) -> Output<ClPointer<Obj2>> {
//     let output_count: u32 = cl_get_object_count(cl_object, flag, func)?;

//     if output_count == 0 {
//         return Ok(ClPointer::new_empty());
//     }

//     let output_size = output_count * (std::mem::size_of::<Obj2>() as u32);
//     let mut bytes = utils::vec_filled_with(0u8, output_size as usize);
//     let output = bytes.as_mut_ptr() as *mut _ as *mut Obj2;

//     let err_code = func(cl_object, flag, output_count, output, std::ptr::null_mut());

//     StatusCodeError::check(err_code)?;
//     // everything worked, but we dont want the `bytes` vec to be dropped so we forget it.
//     std::mem::forget(bytes);
//     Ok(ClPointer::new(output_count as usize, output))
// }

// type InfoFunc5<Obj, Flag> =
//     unsafe extern "system" fn(Obj, Flag, size_t, *mut c_void, *mut size_t) -> cl_int;

// pub unsafe fn cl_get_info_byte_count5<Obj: Copy, Flag: Copy>(
//     cl_object: Obj,
//     flag: Flag,
//     func: InfoFunc5<Obj, Flag>,
// ) -> Output<size_t> {
//     let mut output_size = 0 as size_t;

//     let err_code = func(
//         cl_object,
//         flag,
//         0 as size_t,
//         std::ptr::null_mut(),
//         &mut output_size as *mut size_t,
//     );

//     StatusCodeError::check(err_code)?;
//     Ok(output_size)
// }

// pub unsafe fn cl_get_info5<Obj: Copy, Flag: Copy, Ret: Copy>(
//     cl_object: Obj,
//     flag: Flag,
//     func: InfoFunc5<Obj, Flag>,
// ) -> Output<ClPointer<Ret>> {
//     let num_bytes: size_t = cl_get_info_byte_count5(cl_object, flag, func)?;

//     if num_bytes == 0 {
//         return Ok(ClPointer::new_empty());
//     }

//     let mut bytes = utils::vec_filled_with(0u8, num_bytes as usize);

//     let output = bytes.as_mut_ptr() as *mut _ as *mut libc::c_void;

//     let err_code = func(cl_object, flag, num_bytes, output, std::ptr::null_mut());

//     StatusCodeError::check(err_code)?;
//     // Everything above worked so we don't want the `bytes` vec to be freed
//     // Therefore we forget it.
//     std::mem::forget(bytes);

//     let output_count = num_bytes / std::mem::size_of::<Ret>();
//     Ok(ClPointer::new(output_count, output as *mut Ret))
// }

// type InfoFunc6<Obj, Obj2, Flag> =
//     unsafe extern "system" fn(Obj, Obj2, Flag, size_t, *mut c_void, *mut size_t) -> cl_int;

// pub unsafe fn cl_get_info_byte_count6<Obj1: Copy, Obj2: Copy, Flag: Copy>(
//     cl_obj1: Obj1,
//     cl_obj2: Obj2,
//     flag: Flag,
//     func: InfoFunc6<Obj1, Obj2, Flag>,
// ) -> Output<size_t> {
//     let mut output_size = 0 as size_t;

//     let err_code = func(
//         cl_obj1,
//         cl_obj2,
//         flag,
//         0 as size_t,
//         std::ptr::null_mut(),
//         &mut output_size as *mut size_t,
//     );

//     StatusCodeError::check(err_code)?;
//     Ok(output_size)
// }

// pub unsafe fn cl_get_info6<Obj1: Copy, Obj2: Copy, Flag: Copy, Ret: Copy>(
//     cl_obj1: Obj1,
//     cl_obj2: Obj2,
//     flag: Flag,
//     func: InfoFunc6<Obj1, Obj2, Flag>,
// ) -> Output<ClPointer<Ret>> {
//     let byte_count: size_t = cl_get_info_byte_count6(cl_obj1, cl_obj2, flag, func)?;

//     if byte_count == 0 {
//         return Ok(ClPointer::new_empty());
//     }

//     let mut bytes = utils::vec_filled_with(0u8, byte_count as usize);
//     let output = bytes.as_mut_ptr() as *mut _ as *mut libc::c_void;

//     let err_code = func(
//         cl_obj1,
//         cl_obj2,
//         flag,
//         byte_count,
//         output,
//         std::ptr::null_mut(),
//     );

//     StatusCodeError::check(err_code)?;
//     // Everything above worked so we don't want the `bytes` vec to be freed
//     // Therefore we forget it.
//     std::mem::forget(bytes);
//     let output_count = byte_count / std::mem::size_of::<Ret>();
//     Ok(ClPointer::new(output_count, output as *mut Ret))
// }
