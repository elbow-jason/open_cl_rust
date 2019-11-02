use crate::*;

#[allow(unused_macros)]
macro_rules! expect (
    ($test: expr, $expected: expr) => ({
            let test = $test;
            let expected = $expected;
            if test != expected {
                panic!(
                    "FAILED assertion: {} == {}: expected {:?}, got {:?}",
                    stringify!($test),
                    stringify!($expected),
                    expected,
                    test
                )
            }
        }
    )
);

pub fn test_all<F>(test: &mut F)
where
    F: FnMut(&Device, &Context, &CommandQueue),
{
    println!("Starting test all");
    let platforms = Platform::all().unwrap();
    println!("Got platforms: {:?}", platforms);
    for p in platforms.iter() {
        let devices: Vec<Device> = p
            .all_devices()
            .unwrap()
            .into_iter()
            .filter(|d| {
                let is_usable = d.is_usable();
                if !is_usable {
                    println!("Unusable device found {:?}", d);
                }
                is_usable
            })
            .collect();
        println!("Got devices: {:?}", devices);

        assert!(devices.len() > 0, "No usable devices found");
        // println!("Number of devices: {:?}", devices.len());
        // println!("Devices: {:?}", devices);
        for device in devices.iter() {
            println!("Using device: {:?}", device);
            // println!("starting device: {:?}", i);
            // println!("creating context: {:?}", i);
            let context = Context::create(device).unwrap();
            println!("created context : {:?}", context);
            // println!("created context: {:?}", i);
            // println!("creating command queue: {:?}", i);
            let queue = CommandQueue::create(&context, device, None).unwrap();
            println!("created queue : {:?}", queue);
            // println!("created command queue: {:?}", i);
            test(device, &context, &queue);
        }
    }
}

mod mem {
    // use std::slice;
    // use opencl::mem::{Read, Write};

    // fn read_write<W: Write, R: Read>(src: &W, dst: &mut R)
    // {
    //     // find the max size of the input buffer
    //     let mut max = 0;
    //     src.write(|off, _, len| {
    //         if max < off + len {
    //             max = off + len;
    //         }
    //     });
    //     let max = max as usize;

    //     let mut buffer: Vec<u8> = Vec::new();
    //     unsafe {
    //         buffer.reserve(max);
    //         buffer.set_len(max);
    //     }

    //     // copy from input into buffer
    //     src.write(|off, ptr, len| {
    //         let off = off as usize;
    //         let len = len as usize;
    //         assert!(buffer.len() >= (off + len) as usize);
    //         let target = &mut buffer[off .. off + len];
    //         unsafe {
    //             let ptr = ptr as *const u8;
    //             let src = slice::from_raw_parts(ptr, len);
    //             target.copy_from_slice(src);
    //         }
    //     });

    //     // copy from buffer into output
    //     dst.read(|off, ptr, len| {
    //         let off = off as usize;
    //         let len = len as usize;
    //         assert!(buffer.len() >= (off + len) as usize);
    //         let src = &buffer[off .. off + len];
    //         unsafe {
    //             let ptr = ptr as *mut u8;
    //             let dst = slice::from_raw_parts_mut(ptr, len);
    //             dst.copy_from_slice(src);
    //         }
    //     })
    // }

    // #[test]
    // fn read_write_slice()
    // {
    //     let input: &[isize] = &[0, 1, 2, 3, 4, 5, 6, 7];
    //     let mut output: &mut [isize] = &mut [0, 0, 0, 0, 0, 0, 0, 0];
    //     read_write(&input, &mut output);
    //     expect!(input, output);
    // }

    // #[test]
    // fn read_write_int()
    // {
    //     let input: isize      = 3141;
    //     let mut output: isize = 0;
    //     read_write(&input, &mut output);
    //     expect!(input, output);
    // }

    // #[test]
    // fn read_write_uint()
    // {
    //     let input : usize = 3141;
    //     let mut output : usize = 0;
    //     read_write(&input, &mut output);
    //     expect!(input, output);
    // }

    // #[test]
    // fn read_write_f32()
    // {
    //     let input : f32 = 3141.;
    //     let mut output : f32 = 0.;
    //     read_write(&input, &mut output);
    //     expect!(input, output);
    // }

    // #[test]
    // fn read_write_f64()
    // {
    //     let input : f64 = 3141.;
    //     let mut output : f64 = 0.;
    //     read_write(&input, &mut output);
    //     expect!(input, output);
    // }
}

#[cfg(test)]
mod testing {
    use super::test_all;
    use crate::*;
    #[test]
    fn program_build_mid_level() {
        let src = "__kernel void test(__global int *i) { \
                   *i += 1; \
                   }";
        test_all(&mut |device, context, _| {
            let prog = Program::create_with_source(context, src.to_string()).unwrap();
            prog.build_on_one_device(&device).unwrap();
        })
    }

    #[test]
    fn simple_kernel_test() {
        let src = "
        __kernel void test(__global int *i) {
            *i += 1;
        }";
        use event::event_info::CommandExecutionStatus;

        test_all(&mut |device, context, queue| {
            println!("here 0");
            let program = Program::create_with_source(context, src.to_string()).unwrap();
            println!("here 1");
            program.build_on_one_device(&device).unwrap();
            println!("here 2");
            let k = Kernel::create(&program, "test").unwrap();
            println!("here 3");
            let mut v1: Vec<isize> = vec![1];
            println!("here 4");
            let mem1 = DeviceMem::create_read_write(context, v1.len()).unwrap();
            println!("here 5");
            let work_size = v1.len();
            println!("here 6");
            let work: Work = Work::new(work_size);
            println!("here 7");
            let write_event = queue.write_buffer(&mem1, &v1).unwrap();

            assert!(write_event.command_execution_status() == Ok(CommandExecutionStatus::Complete));

            println!("here 8");
            let () = k.set_arg(0, &mem1).unwrap();
            println!("here 9");
            let _queue_event: Event = queue
                .sync_enqueue_kernel(&k, work)
                .unwrap_or_else(|error| {
                    panic!("Failed to unwrap sync_enqueue_kernel result: {:?}", error);
                });

            let _read_event = queue.read_buffer(&mem1, &mut v1).unwrap();

            expect!(v1.len(), 1);
            expect!(v1[0], 2);
        })
    }

    #[test]
    fn add_scalar_int_var_to_buffer_test() {
        let src = "
        __kernel void test(__global int *i, long int num) {
            *i += num;
        }";

        test_all(&mut |device, context, queue| {
            let program = Program::create_with_source(context, src.to_string()).unwrap();
            program.build_on_one_device(device).unwrap();

            let add_scalar_var: Kernel = Kernel::create(&program, "test").unwrap();
            let initial_values = vec![1i32];
            let mem1 = DeviceMem::create_write_only(context, initial_values.len()).unwrap();
            let _write_event = queue.write_buffer(&mem1, &initial_values[..]).unwrap();

            let () = add_scalar_var.set_arg(0, &mem1).unwrap();

            let arg = 42i32;
            let () = add_scalar_var.set_arg(1, &arg).unwrap();

            let work_size = initial_values.len();
            let work: Work = Work::new(work_size);
            let _queue_event: Event = queue.sync_enqueue_kernel(&add_scalar_var, work)
                .unwrap_or_else(|error| {
                    panic!("Failed to unwrap sync_enqueue_kernel result: {:?}", error);
                });
            let mut result = vec![0i32];
            let _write_event = queue.read_buffer(&mem1, &mut result[..]).unwrap();

            expect!(initial_values[0], 1);
            expect!(result[0], 43);
            expect!(initial_values.len(), result.len());
        })
    }

    // #[test]
    // fn chain_kernel_event() {
    //     let src = "__kernel void test(__global int *i) { \
    //                *i += 1; \
    //                }";

    //     ::test_all_platforms_devices(&mut |device, ctx, queue| {
    //         let prog = ctx.create_program_from_source(src);
    //         prog.build(device).unwrap();

    //         let k = prog.create_kernel("test");
    //         let v = ctx.create_buffer_from(vec![1isize], CL_MEM_READ_WRITE);

    //         k.set_arg(0, &v);

    //         let mut e : Option<Event> = None;
    //         for _ in 0isize .. 8 {
    //             e = Some(queue.enqueue_async_kernel(&ctx, &k, 1isize, None, e));
    //         }
    //         e.wait();

    //         let v: Vec<isize> = queue.get(&v, ());

    //         expect!(v[0], 9);
    //     })
    // }

    //     #[test]
    //     fn chain_kernel_event_list() {
    //         let src = "__kernel void inc(__global int *i) { \
    //                    *i += 1; \
    //                    } \
    //                    __kernel void add(__global int *a, __global int *b, __global int *c) { \
    //                    *c = *a + *b; \
    //                    }";

    //         ::test_all_platforms_devices(&mut |device, ctx, queue| {
    //             let prog = ctx.create_program_from_source(src);
    //             prog.build(device).unwrap();

    //             let k_inc_a = prog.create_kernel("inc");
    //             let k_inc_b = prog.create_kernel("inc");
    //             let k_add = prog.create_kernel("add");

    //             let a = ctx.create_buffer_from(vec![1isize], CL_MEM_READ_WRITE);
    //             let b = ctx.create_buffer_from(vec![1isize], CL_MEM_READ_WRITE);
    //             let c = ctx.create_buffer_from(vec![1isize], CL_MEM_READ_WRITE);

    //             k_inc_a.set_arg(0, &a);
    //             k_inc_b.set_arg(0, &b);

    //             let event_list = [
    //                 queue.enqueue_async_kernel(&ctx, &k_inc_a, 1isize, None, ()),
    //                 queue.enqueue_async_kernel(&ctx, &k_inc_b, 1isize, None, ()),
    //             ];

    //             k_add.set_arg(0, &a);
    //             k_add.set_arg(1, &b);
    //             k_add.set_arg(2, &c);

    //             let event = queue.enqueue_async_kernel(&ctx, &k_add, 1isize, None, &event_list[..]);

    //             let v: Vec<isize> = queue.get(&c, event);

    //             expect!(v[0], 4);
    //         })
    //     }

    // #[test]
    // fn kernel_2d() {
    //     let src = "__kernel void test(__global long int *N) {
    //                int i = get_global_id(0);
    //                int j = get_global_id(1);
    //                int s = get_global_size(0);
    //                N[i * s + j] = i * j;
    //     }";
    //     test_all(&mut |device, context, queue| {
    //         let prog = Program::create_with_source(context, src.to_string()).unwrap();

    //         let () = prog.build_on_one_device(device).unwrap();

    //         let k = prog.fetch_kernel("test").unwrap();
    //         let v1 = vec![1isize, 2, 3, 4, 5, 6, 7, 8, 9];
    //         let b1 = DeviceMem::create_read_only(context, v1.len()).unwrap();
    //         let work = Work::new((3, 3));
    //         let () = k.set_arg(0, &b1).unwrap();

    //         let _kernel_event = queue
    //             .sync_enqueue_kernel(&k, work, WaitList::empty())
    //             .unwrap();

    //         let mut v2 = vec![0; v1.len()]; // utils::vec_filled_with(0, v1.len());
    //         let _event: Event = queue
    //             .read_buffer(&b1, &mut v2, WaitList::empty(), None)
    //             .unwrap();

    //         expect!(v2, vec!(0, 0, 0, 0, 1, 2, 0, 2, 4));
    //     })
    // }

    #[test]
    fn memory_read_write_test() {
        test_all(&mut |_, context, queue| {
            let buffer: DeviceMem<isize> = DeviceMem::create_read_only(context, 8).unwrap();

            let input = [0isize, 1, 2, 3, 4, 5, 6, 7];
            let mut output = [0isize, 0, 0, 0, 0, 0, 0, 0];

            let _write_event = queue.write_buffer(&buffer, &input[..]).unwrap();
            let _read_event = queue.read_buffer(&buffer, &mut output[..]).unwrap();

            expect!(input, output);
        })
    }

    // #[test]
    // fn memory_read_vec_test() {
    //     test_all(&mut |_, ctx, queue| {
    //         let input = [0isize, 1, 2, 3, 4, 5, 6, 7];
    //         let buffer = ctx.create_read_write_buffer(input.len());
    //         let output: Vec<isize> = queue.get(&buffer, ());
    //         expect!(&input[..], &output[..]);
    //     })
    // }

    //     #[test]
    //     fn memory_read_owned()
    //     {
    //         ::test_all_platforms_devices(&mut |_, ctx, queue| {
    //             let input = vec!(0isize, 1, 2, 3, 4, 5, 6, 7);
    //             let buffer = ctx.create_buffer_from(&input, CL_MEM_READ_WRITE);
    //             let output: Vec<isize> = queue.get(&buffer, ());
    //             expect!(input, output);
    //         })
    //     }

        // #[test]
        // fn memory_read_owned_clone()
        // {
        //     test_all(&mut |_, ctx, queue| {
        //         let input = vec!(0isize, 1, 2, 3, 4, 5, 6, 7);
        //         let buffer = DeviceMem::create_read_write_from(ctx, &input)
        //             .expect("create_read_write_from failed");
        //         let mut output = utils::vec_filled_with(0, input.len());
        //         let _e1 = queue.read_buffer(&buffer, &mut output)
        //             .expect("read_buffer failed");
        //         expect!(input, output);
        //     })
        // }

    //     #[test]
    //     fn event_get_times() {
    //         let src = "__kernel void test(__global int *i) { \
    //                    *i += 1; \
    //                    }";

    //         let (device, ctx, queue) = util::create_compute_context().unwrap();
    //         let prog = ctx.create_program_from_source(src);
    //         prog.build(&device).unwrap();

    //         let k = prog.create_kernel("test");
    //         let v = ctx.create_buffer_from(vec![1isize], CL_MEM_READ_WRITE);

    //         k.set_arg(0, &v);

    //         let e = queue.enqueue_async_kernel(&ctx, &k, 1isize, None, ());
    //         e.wait();

    //         // the that are returned are not useful for unit test, this test
    //         // is mostly testing that opencl returns no error
    //         e.queue_time();
    //         e.submit_time();
    //         e.start_time();
    //         e.end_time();
    //     }
}

// #[cfg(test)]
// mod array {
//     use opencl::array::*;
//     use opencl::cl::CL_MEM_READ_WRITE;

//     #[test]
//     fn put_get_2d()
//     {
//         ::test_all_platforms_devices(&mut |_, ctx, queue| {
//             let arr_in = Array2D::new(8, 8, |x, y| {(x+y) as isize});
//             let arr_cl = ctx.create_buffer_from(&arr_in, CL_MEM_READ_WRITE);
//             let arr_out: Array2D<isize> = queue.get(&arr_cl, ());

//             for x in 0usize.. 8usize {
//                 for y in 0usize..8usize {
//                     expect!(arr_in.get(x, y), arr_out.get(x, y));
//                 }
//             }
//         })
//     }

//     #[test]
//     fn read_write_2d()
//     {
//         ::test_all_platforms_devices(&mut |_, ctx, queue| {
//             let added = Array2D::new(8, 8, |x, y| {(x+y) as isize});
//             let zero = Array2D::new(8, 8, |_, _| {(0) as isize});
//             let mut out = Array2D::new(8, 8, |_, _| {(0) as isize});

//             /* both are zeroed */
//             let a_cl = ctx.create_buffer_from(&zero, CL_MEM_READ_WRITE);

//             queue.write(&a_cl, &added, ());
//             queue.read(&a_cl, &mut out, ());

//             for x in 0usize .. 8usize {
//                 for y in 0usize .. 8usize {
//                     expect!(added.get(x, y), out.get(x, y));
//                 }
//             }
//         })
//     }

//     #[test]
//     fn kernel_2d()
//     {
//         ::test_all_platforms_devices(&mut |device, ctx, queue| {
//             let mut a = Array2D::new(8, 8, |_, _| {(0) as i32});
//             let b = Array2D::new(8, 8, |x, y| {(x*y) as i32});
//             let a_cl = ctx.create_buffer_from(&a, CL_MEM_READ_WRITE);

//             let src =  "__kernel void test(__global int *a) { \
//                             int x = get_global_id(0); \
//                             int y = get_global_id(1); \
//                             int size_x = get_global_size(0); \
//                             a[size_x*y + x] = x*y; \
//                         }";
//             let prog = ctx.create_program_from_source(src);
//             match prog.build(device) {
//                 Ok(_) => (),
//                 Err(build_log) => {
//                     println!("Error building program:\n");
//                     println!("{}", build_log);
//                     panic!("");
//                 }
//             }
//             let k = prog.create_kernel("test");

//             k.set_arg(0, &a_cl);
//             let event = queue.enqueue_async_kernel(&ctx, &k, (8isize, 8isize), None, ());
//             queue.read(&a_cl, &mut a, &event);

//             for x in 0usize .. 8usize {
//                 for y in 0usize .. 8usize {
//                     expect!(a.get(x, y), b.get(x, y));
//                 }
//             }
//         })
//     }

//     /*#[test]
//     fn kernel_2d_offset()
//     {
//         ::test_all_platforms_devices(&mut |device, ctx, queue| {
//             let mut a = Array2D::new(8, 8, |_, _| {(1) as i32});
//             let b = Array2D::new(8, 8, |x, y| {(x*y) as i32});
//             let a_cl = ctx.create_buffer_from(&a, CL_MEM_READ_WRITE);

//             let src =  "__kernel void test(__global int *a, ulong size_x) { \
//                             int x_off = get_global_offset(0); \
//                             int x = get_global_id(0) + x_off; \
//                             int y_off = get_global_offset(1); \
//                             int y = get_global_id(1) + y_off; \
//                             a[size_x*y + x] = x*y; \
//                         }";
//             let prog = ctx.create_program_from_source(src);
//             match prog.build(device) {
//                 Ok(_) => (),
//                 Err(build_log) => {
//                     println!("Error building program:\n");
//                     println!("{}", build_log);
//                     panic!("");
//                 }
//             }
//             let k = prog.create_kernel("test");

//             k.set_arg(0, &a_cl);
//             k.set_arg(1, &a.width());
//             let event = queue.enqueue_async_kernel(&ctx, &k, Some((3, 3)), (5isize, 5isize), None, ());
//             queue.read(&a_cl, &mut a, &event);

//             println!("");
//             for y in 0usize .. 8usize {
//                 for x in 0usize .. 8usize {
//                     let _a = a.get(x, y);
//                     print!("{:?}\t", _a);
//                     if x < 3 || y < 3 {
//                         expect!(a.get(x, y), 1);
//                     } else {
//                         expect!(a.get(x, y), b.get(x, y));
//                     }
//                 }
//                 println!("");
//             }
//         })
//     }*/
//     #[test]
//     fn put_get_3d()
//     {
//         ::test_all_platforms_devices(&mut |_, ctx, queue| {
//             let arr_in = Array3D::new(8, 8, 8, |x, y, z| {(x+y+z) as isize});
//             let arr_cl = ctx.create_buffer_from(&arr_in, CL_MEM_READ_WRITE);
//             let arr_out: Array3D<isize> = queue.get(&arr_cl, ());

//             for x in 0usize .. 8usize {
//                 for y in 0usize .. 8usize {
//                     for z in 0usize .. 8usize {
//                         expect!(arr_in.get(x, y, z), arr_out.get(x, y, z));
//                     }
//                 }
//             }
//         })
//     }

//     #[test]
//     fn read_write_3d()
//     {
//         ::test_all_platforms_devices(&mut |_, ctx, queue| {
//             let added = Array3D::new(8, 8, 8, |x, y, z| {(x+y+z) as isize});
//             let zero = Array3D::new(8, 8, 8, |_, _, _| {(0) as isize});
//             let mut out = Array3D::new(8, 8, 8, |_, _, _| {(0) as isize});

//             /* both are zeroed */
//             let a_cl = ctx.create_buffer_from(&zero, CL_MEM_READ_WRITE);

//             queue.write(&a_cl, &added, ());
//             queue.read(&a_cl, &mut out, ());

//             for x in 0usize .. 8usize {
//                 for y in 0usize .. 8usize {
//                     for z in 0usize .. 8usize {
//                         expect!(added.get(x, y, z), out.get(x, y, z));
//                     }
//                 }
//             }
//         })
//     }

//     #[test]
//     fn kernel_3d()
//     {
//         ::test_all_platforms_devices(&mut |device, ctx, queue| {
//             let mut a = Array3D::new(8, 8, 8, |_, _, _| {(0) as i32});
//             let b = Array3D::new(8, 8, 8, |x, y, z| {(x*y*z) as i32});
//             let a_cl = ctx.create_buffer_from(&a, CL_MEM_READ_WRITE);

//             let src =  "__kernel void test(__global int *a) { \
//                             int x = get_global_id(0); \
//                             int y = get_global_id(1); \
//                             int z = get_global_id(2); \
//                             int size_x = get_global_size(0); \
//                             int size_y = get_global_size(1); \
//                             a[size_x*size_y*z + size_x*y + x] = x*y*z; \
//                         }";
//             let prog = ctx.create_program_from_source(src);
//             match prog.build(device) {
//                 Ok(_) => (),
//                 Err(build_log) => {
//                     println!("Error building program:\n");
//                     println!("{}", build_log);
//                     panic!("");
//                 }
//             }
//             let k = prog.create_kernel("test");

//             k.set_arg(0, &a_cl);
//             let event = queue.enqueue_async_kernel(&ctx, &k, (8isize, 8isize, 8isize), None, ());
//             queue.read(&a_cl, &mut a, &event);

//             for x in 0usize .. 8usize {
//                 for y in 0usize .. 8usize {
//                     for z in 0usize .. 8usize {
//                         expect!(a.get(x, y, z), b.get(x, y, z));
//                     }
//                 }
//             }
//         })
//     }
// }

// #[cfg(test)]
// mod ext {
//     use opencl::ext;
//     use opencl::hl::*;

//     #[test]
//     fn try_load_all_extensions() {
//         let platforms = get_platforms();

//         for platform in platforms.into_iter() {
//             let platform_id = platform.get_id();

//             macro_rules! check_ext {
//                 ($ext:ident) => {
//                     match ext::$ext::load(platform_id) {
//                         Ok(_) => {
//                             info!("Extension {} loaded successfully.",
//                                   stringify!($ext))
//                         }
//                         Err(_) => {
//                             info!("Error loading extension {}.",
//                                   stringify!($ext))
//                         }
//                     }
//                 }
//             }

//             check_ext!(cl_khr_fp64);
//             check_ext!(cl_khr_fp16);
//             check_ext!(cl_APPLE_SetMemObjectDestructor);
//             check_ext!(cl_APPLE_ContextLoggingFunctions);
//             check_ext!(cl_khr_icd);
//             check_ext!(cl_nv_device_attribute_query);
//             check_ext!(cl_amd_device_attribute_query);
//             check_ext!(cl_arm_printf);
//             check_ext!(cl_ext_device_fission);
//             check_ext!(cl_qcom_ext_host_ptr);
//             check_ext!(cl_qcom_ion_host_ptr);
//         }
//     }
// }

// #[cfg(test)]
// mod cl {
//     use opencl::cl::CLStatus::*;

//     #[test]
//     fn clstatus_str() {
//         let x = CL_SUCCESS;
//         expect!(format!("{}", x), "CL_SUCCESS");

//         let y = CL_DEVICE_NOT_FOUND;
//         expect!(y.to_string(), "CL_DEVICE_NOT_FOUND");
//     }
// }
