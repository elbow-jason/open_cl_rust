// use std::marker::PhantomData;
// use std::fmt;

pub use crate::{build_output, utils, ClPointer, Output, StatusCodeError};

use crate::ffi::cl_int;

use libc::{c_void, size_t};

type ObjFunc<Obj, Flag, Obj2> = unsafe extern "C" fn(Obj, Flag, u32, *mut Obj2, *mut u32) -> cl_int;

pub unsafe fn cl_get_object_count<Obj, Flag, Obj2>(
    cl_object: Obj,
    flag: Flag,
    func: ObjFunc<Obj, Flag, Obj2>,
) -> Output<u32>
where
    Obj: Copy,
    Flag: Copy,
    Obj2: Copy,
{
    let mut output_size: u32 = 0;

    let err_code = func(
        cl_object,
        flag,
        0 as u32,
        std::ptr::null_mut(),
        &mut output_size as *mut u32,
    );

    build_output(output_size, err_code)
}

pub unsafe fn cl_get_object<Obj: Copy, Flag: Copy, Obj2: Copy>(
    cl_object: Obj,
    flag: Flag,
    func: ObjFunc<Obj, Flag, Obj2>,
) -> Output<ClPointer<Obj2>> {
    let output_count: u32 = cl_get_object_count(cl_object, flag, func)?;

    if output_count == 0 {
        return Ok(ClPointer::new_empty());
    }

    let output_size = output_count * (std::mem::size_of::<Obj2>() as u32);
    let mut bytes = utils::vec_filled_with(0u8, output_size as usize);
    let output = bytes.as_mut_ptr() as *mut _ as *mut Obj2;

    let err_code = func(cl_object, flag, output_count, output, std::ptr::null_mut());

    build_output((), err_code)?;
    // everything worked, but we dont want the `bytes` vec to be dropped so we forget it.
    std::mem::forget(bytes);
    Ok(ClPointer::new(output_count as usize, output))
}

type InfoFunc5<Obj, Flag> =
    unsafe extern "C" fn(Obj, Flag, size_t, *mut c_void, *mut size_t) -> cl_int;

pub unsafe fn cl_get_info_byte_count5<Obj: Copy, Flag: Copy>(
    cl_object: Obj,
    flag: Flag,
    func: InfoFunc5<Obj, Flag>,
) -> Output<size_t> {
    let mut output_size = 0 as size_t;

    let err_code = func(
        cl_object,
        flag,
        0 as size_t,
        std::ptr::null_mut(),
        &mut output_size as *mut size_t,
    );

    build_output(output_size, err_code)
}

pub unsafe fn cl_get_info5<Obj: Copy, Flag: Copy, Ret: Copy>(
    cl_object: Obj,
    flag: Flag,
    func: InfoFunc5<Obj, Flag>,
) -> Output<ClPointer<Ret>> {
    let num_bytes: size_t = cl_get_info_byte_count5(cl_object, flag, func)?;

    if num_bytes == 0 {
        return Ok(ClPointer::new_empty());
    }

    let mut bytes = utils::vec_filled_with(0u8, num_bytes as usize);

    let output = bytes.as_mut_ptr() as *mut _ as *mut libc::c_void;

    let err_code = func(cl_object, flag, num_bytes, output, std::ptr::null_mut());

    build_output((), err_code)?;
    // Everything above worked so we don't want the `bytes` vec to be freed
    // Therefore we forget it.
    std::mem::forget(bytes);

    let output_count = num_bytes / std::mem::size_of::<Ret>();
    Ok(ClPointer::new(output_count, output as *mut Ret))
}

type InfoFunc6<Obj, Obj2, Flag> =
    unsafe extern "C" fn(Obj, Obj2, Flag, size_t, *mut c_void, *mut size_t) -> cl_int;

pub unsafe fn cl_get_info_byte_count6<Obj1: Copy, Obj2: Copy, Flag: Copy>(
    cl_obj1: Obj1,
    cl_obj2: Obj2,
    flag: Flag,
    func: InfoFunc6<Obj1, Obj2, Flag>,
) -> Output<size_t> {
    let mut output_size = 0 as size_t;

    let err_code = func(
        cl_obj1,
        cl_obj2,
        flag,
        0 as size_t,
        std::ptr::null_mut(),
        &mut output_size as *mut size_t,
    );

    build_output(output_size, err_code)
}

pub unsafe fn cl_get_info6<Obj1: Copy, Obj2: Copy, Flag: Copy, Ret: Copy>(
    cl_obj1: Obj1,
    cl_obj2: Obj2,
    flag: Flag,
    func: InfoFunc6<Obj1, Obj2, Flag>,
) -> Output<ClPointer<Ret>> {
    let byte_count: size_t = cl_get_info_byte_count6(cl_obj1, cl_obj2, flag, func)?;

    if byte_count == 0 {
        return Ok(ClPointer::new_empty());
    }

    let mut bytes = utils::vec_filled_with(0u8, byte_count as usize);
    let output = bytes.as_mut_ptr() as *mut _ as *mut libc::c_void;

    let err_code = func(
        cl_obj1,
        cl_obj2,
        flag,
        byte_count,
        output,
        std::ptr::null_mut(),
    );

    build_output((), err_code)?;
    // Everything above worked so we don't want the `bytes` vec to be freed
    // Therefore we forget it.
    std::mem::forget(bytes);
    let output_count = byte_count / std::mem::size_of::<Ret>();
    Ok(ClPointer::new(output_count, output as *mut Ret))
}
