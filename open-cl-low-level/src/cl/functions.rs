use std::sync::Mutex;

use crate::ffi::{
    clCreateContext, clGetContextInfo, clGetDeviceIDs, clGetDeviceInfo, clGetPlatformIDs,
    clGetPlatformInfo, cl_context_info, cl_device_info, cl_device_type, cl_platform_info, cl_uint,
};
use crate::{Output, StatusCodeError};

use crate::cl::{cl_context, cl_device_id, cl_platform_id, ClObject};
use crate::strings;
use libc::{c_void, size_t};

macro_rules! _object_count {
    ($func:ident, $parent:ident, $flag:ident) => {{
        let mut output_size: u32 = 0;
        let err_code = $func(
            $parent.as_ptr() as *mut c_void,
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
                $parent.as_ptr() as *mut c_void,
                $flag,
                $count,
                output.as_mut_ptr() as *mut *mut c_void,
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

const SIZE_OF_USIZE: usize = std::mem::size_of::<usize>();
const SIZE_OF_U64: usize = std::mem::size_of::<u64>();

macro_rules! _empty_data {
    (String, $n_bytes:expr) => {
        vec![0u8; $n_bytes]
    };

    (Vec_usize, $n_bytes:expr) => {{
        assert_eq!($n_bytes % SIZE_OF_USIZE, 0);
        vec![0usize; $n_bytes / SIZE_OF_USIZE]
    }};
    (Vec_u64, $n_bytes:expr) => {{
        assert_eq!($n_bytes % SIZE_OF_U64, 0);
        vec![0u64; $n_bytes / SIZE_OF_U64]
    }};

    (bool, $n_bytes:expr) => {{
        assert_eq!($n_bytes, 4);
        0u32
    }};
    (Vec_device, $n_bytes:expr) => {{
        assert_eq!($n_bytes, SIZE_OF_USIZE);
        debug_assert!(std::mem::size_of::<cl_device_id>() == std::mem::size_of::<usize>());
        vec![cl_device_id::null_ptr(); $n_bytes / SIZE_OF_USIZE]
    }};
    ($t:ident, $n_bytes:expr) => {{
        assert_eq!($n_bytes, std::mem::size_of::<$t>());
        0 as $t
    }};
}

macro_rules! _as_ptr {
    (String, $data:expr) => {
        // $data is a Vec<u8> here
        $data.as_mut_ptr() as *mut c_void
    };
    (Vec_usize, $data:expr) => {
        // $data is a Vec<usize> here
        $data.as_mut_ptr() as *mut c_void
    };
    (Vec_u64, $data:expr) => {
        // $data is a Vec<usize> here
        $data.as_mut_ptr() as *mut c_void
    };

    (Vec_device, $data:expr) => {
        // $data is a Vec<cl_device_id> here
        $data.as_mut_ptr() as *mut c_void
    };
    ($t:ident, $data:expr) => {
        // $data is a u32 here
        &mut $data as *mut _ as *mut c_void
    };
}

macro_rules! _finalize_data {
    (String, $data:expr) => {
        strings::to_utf8_string($data)
    };
    (bool, $data:expr) => {
        match $data {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    };
    ($t:ident, $data:expr) => {
        $data
    };
}

macro_rules! _get_info {
    ($output_t:ident, $func:ident, $parent:ident, $flag:ident) => {{
        let mut n_bytes = 0usize;
        let status_code = $func(
            $parent.as_ptr() as *mut c_void,
            $flag,
            0 as size_t,
            std::ptr::null_mut(),
            &mut n_bytes,
        );
        StatusCodeError::check(status_code)?;
        let mut data = _empty_data!($output_t, n_bytes);
        let status = $func(
            $parent.as_ptr() as *mut c_void,
            $flag,
            n_bytes,
            _as_ptr!($output_t, data),
            std::ptr::null_mut(),
        );
        StatusCodeError::check(status)?;
        Ok(_finalize_data!($output_t, data))
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

lazy_static! {
    static ref PLATFORM_ACCESS: Mutex<()> = Mutex::new(());
}

/// Gets the cl_platform_ids of the host machine
pub unsafe fn list_platform_ids() -> Output<Vec<cl_platform_id>> {
    let platform_lock = PLATFORM_ACCESS.lock();
    // transactional access to the platform Mutex requires a lock for some OpenCL implementations.
    let mut num_platforms: cl_uint = 0;
    StatusCodeError::check(clGetPlatformIDs(
        0,
        std::ptr::null_mut(),
        &mut num_platforms,
    ))?;
    let mut ids: Vec<cl_platform_id> = vec![cl_platform_id::null_ptr(); num_platforms as usize];
    StatusCodeError::check(clGetPlatformIDs(
        num_platforms,
        ids.as_mut_ptr() as *mut *mut c_void,
        &mut num_platforms,
    ))?;
    std::mem::drop(platform_lock);
    Ok(ids)
}

#[inline(always)]
pub unsafe fn device_count(platform: cl_platform_id, device_type: cl_device_type) -> Output<u32> {
    _object_count!(clGetDeviceIDs, platform, device_type)
}

#[inline(always)]
pub unsafe fn list_devices(
    platform: cl_platform_id,
    device_type: cl_device_type,
) -> Output<Vec<cl_device_id>> {
    let count = device_count(platform, device_type)?;
    _get_objects!(clGetDeviceIDs, cl_device_id, platform, device_type, count)
}

/// Gets platform info for a given cl_platform_id and the given cl_platform_info flag via the
/// OpenCL FFI call to clGetPlatformInfo.
///
/// # Safety
/// Use of an invalid cl_platform_id is undefined behavior. Be careful. There be dragons.
#[inline(always)]
pub unsafe fn get_platform_info(
    platform: cl_platform_id,
    flag: cl_platform_info,
) -> Output<String> {
    _get_info!(String, clGetPlatformInfo, platform, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_string(device: cl_device_id, flag: cl_device_info) -> Output<String> {
    _get_info!(String, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_bool(device: cl_device_id, flag: cl_device_info) -> Output<bool> {
    _get_info!(bool, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_vec_usize(
    device: cl_device_id,
    flag: cl_device_info,
) -> Output<Vec<usize>> {
    _get_info!(Vec_usize, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_u32(device: cl_device_id, flag: cl_device_info) -> Output<u32> {
    _get_info!(u32, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_u64(device: cl_device_id, flag: cl_device_info) -> Output<u64> {
    _get_info!(u64, clGetDeviceInfo, device, flag)
}

#[inline(always)]
pub unsafe fn get_device_info_usize(device: cl_device_id, flag: cl_device_info) -> Output<usize> {
    _get_info!(usize, clGetDeviceInfo, device, flag)
}

#[allow(clippy::transmuting_null)]
pub unsafe fn create_context(device_ids: &[cl_device_id]) -> Output<cl_context> {
    let mut err_code = 0;
    let context_ptr = clCreateContext(
        std::ptr::null(),
        device_ids.len() as u32,
        device_ids.as_ptr() as *const *mut c_void,
        std::mem::transmute(std::ptr::null::<fn()>()),
        std::ptr::null_mut(),
        &mut err_code,
    );
    StatusCodeError::check(err_code)?;
    cl_context::new(context_ptr)
}

#[inline(always)]
pub unsafe fn get_context_info_u32(context: cl_context, flag: cl_context_info) -> Output<u32> {
    _get_info!(u32, clGetContextInfo, context, flag)
}

#[inline(always)]
pub unsafe fn get_context_info_devices(
    context: cl_context,
    flag: cl_context_info,
) -> Output<Vec<cl_device_id>> {
    _get_info!(Vec_device, clGetContextInfo, context, flag)
}

#[inline(always)]
pub unsafe fn get_context_info_vec_u64(
    context: cl_context,
    flag: cl_context_info,
) -> Output<Vec<u64>> {
    _get_info!(Vec_u64, clGetContextInfo, context, flag)
}

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
