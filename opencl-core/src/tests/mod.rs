use crate::*;
use crate::opencl_low_level::*;

#[test]
fn core_program_unbuilt_to_built() {
    let src = "__kernel void test(__global int *i) { \
                *i += 1; \
                }";
    testing::test_all_devices(&mut |device, context, _| {
        let devices = vec![device.clone()];
        let unbuilt_prog: UnbuiltProgram = UnbuiltProgram::create_with_source(context, src).unwrap();
        let _build: Program = unbuilt_prog.build(&devices[..]).unwrap();
    })
}

#[test]
fn simple_kernel_test() {
    let src = "
    __kernel void test(__global int *i) {
        *i += 1;
    }";

    testing::test_all_devices(&mut |device, context, queue| {
        let unbuilt_program = UnbuiltProgram::create_with_source(context, src).unwrap();
        let devices = vec![device.clone()];
        let program = unbuilt_program.build(&devices[..]).unwrap();
        let k = Kernel::create(&program, "test").unwrap();
        let mut v1: Vec<isize> = vec![1];
        let mem_config = MemConfig::for_size();
        let buffer = Buffer::create(context, v1.len(), mem_config.host_access, mem_config.kernel_access, mem_config.mem_location).unwrap();
        let work_size = v1.len();
        let work: Work = Work::new(work_size);
        queue.write_buffer(&buffer, &v1, None).unwrap();
        let mut buffer_write = buffer.write_lock();
        // assert!(write_event.command_execution_status() == Ok(CommandExecutionStatus::Complete));

        let () = unsafe { k.set_arg(0, &mut *buffer_write).unwrap() };
            queue.enqueue_kernel(k, &work, None)
                .unwrap_or_else(|error| {
                    panic!("Failed to unwrap sync_enqueue_kernel result: {:?}", error);
                });
        std::mem::drop(buffer_write);
        let _read_event = queue.read_buffer(&buffer, &mut v1[..], None).unwrap();

        assert_eq!(v1.len(), 1);
        assert_eq!(v1[0], 2);
    })
}

#[test]
fn add_scalar_int_var_to_buffer_test() {
    let src = "
    __kernel void test(__global int *i, long int num) {
        *i += num;
    }";

    testing::test_all_devices(&mut |device, context, queue| {
        let unbuilt_program = UnbuiltProgram::create_with_source(context, src).unwrap();
        let devices = vec![device.clone()];
        let program = unbuilt_program.build(&devices[..]).unwrap();

        let add_scalar_var: Kernel = Kernel::create(&program, "test").unwrap();
        let initial_values = vec![1i32];
        let buffer = Buffer::create_with_creator(context, initial_values.len()).unwrap();
        let _write_event = queue.write_buffer(&buffer, &initial_values[..], None).unwrap();

        
        unsafe {
            let mut mem1 = buffer.write_lock();
            let mut arg = 42i32;
            let () = add_scalar_var.set_arg(0, &mut *mem1).unwrap();
            let () = add_scalar_var.set_arg(1, &mut arg).unwrap();
        }
        let work_size = initial_values.len();
        let work: Work = Work::new(work_size);
        queue.enqueue_kernel(add_scalar_var, &work, None)
            .unwrap_or_else(|error| {
                panic!("Failed to unwrap sync_enqueue_kernel result: {:?}", error);
            });
        let mut result = vec![0i32];
        let _write_event = queue.read_buffer(&buffer, &mut result[..], None).unwrap();

        assert_eq!(initial_values[0], 1);
        assert_eq!(result[0], 43);
        assert_eq!(initial_values.len(), result.len());
    })
}

//     #[test]
//     fn kernel_2d() {
//         let src = "__kernel void test(__global long int *N) {
//                    int i = get_global_id(0);
//                    int j = get_global_id(1);
//                    int s = get_global_size(0);
//                    N[i * s + j] = i * j;
//         }";
//         test_all(&mut |device, context, queue| {
//             let unbuilt_program = UnbuiltProgram::create_with_source(context, src).unwrap();
//             let devices = vec![device];
//             let programs = unbuilt_program
//                 .build(&devices[..])
//                 .expect("failed to build_one_on_device");

//             let k = Kernel::create(&programs[0], "test").expect("failed to create 'test' kernel");
//             let v1 = vec![1isize, 2, 3, 4, 5, 6, 7, 8, 9];
//             let b1 = DeviceMem::create_read_only_from(context, &v1).unwrap();
//             let work = Work::new((3, 3));
//             let () = k.set_arg(0, &b1).expect("failed to set_arg(0, &b1)");

//             let _kernel_event = queue
//                 .sync_enqueue_kernel(&k, &work)
//                 .expect("failed to sync_enqueue_kernel");

//             let mut v2 = vec![0; v1.len()]; // utils::vec_filled_with(0, v1.len());
//             let _event: Event = queue
//                 .read_buffer(&b1, &mut v2)
//                 .expect("failed to read_buffer");

//             expect!(v2, vec!(0, 0, 0, 0, 1, 2, 0, 2, 4));
//         })
//     }

//     #[test]
//     fn memory_read_write_test() {
//         test_all(&mut |_, context, queue| {
//             let input = vec![0isize, 1, 2, 3, 4, 5, 6, 7];
//             let buffer: DeviceMem<isize> =
//                 DeviceMem::create_read_only(context, input.len()).unwrap();

//             let mut output = utils::vec_filled_with(0, input.len());

//             let _write_event = queue.write_buffer(&buffer, &input[..]).unwrap();
//             let _read_event = queue.read_buffer(&buffer, &mut output[..]).unwrap();

//             expect!(input, output);
//         })
//     }

//     #[test]
//     fn memory_read_vec_test() {
//         test_all(&mut |_, context, queue| {
//             let input = vec![0isize, 1, 2, 3, 4, 5, 6, 7];
//             let buffer: DeviceMem<isize> = DeviceMem::create_read_write(context, input.len())
//                 .expect("failed to create_read_write");

//             let mut output = utils::vec_filled_with(0, input.len());

//             let _write_event = queue
//                 .write_buffer(&buffer, &input)
//                 .expect("failed to write_buffer");

//             let _read_event = queue
//                 .read_buffer(&buffer, &mut output)
//                 .expect("failed to read_buffer");

//             expect!(&input[..], &output[..]);
//         })
//     }

//     #[test]
//     fn memory_read_owned_clone() {
//         test_all(&mut |_, ctx, queue| {
//             let input = vec![0isize, 1, 2];
//             let buffer = DeviceMem::create_read_write_from(ctx, &input[..])
//                 .expect("create_read_write_from failed");
//             let mut output = utils::vec_filled_with(0, input.len());
//             let _e1 = queue
//                 .read_buffer(&buffer, &mut output)
//                 .expect("read_buffer failed");
//             expect!(input, output);
//         })
//     }

//     #[test]
//     fn transpose_tensor_2d_test() {
//         let src = "
//         __kernel void transpose_2d(__global const ulong *a,
//                                    __global ulong *b,
//                                    const ulong rows,
//                                    const ulong cols) {
//                 ulong i = get_global_id(0);
//                 ulong j = get_global_id(1);
//                 b[j*rows + i] = a[i*cols + j];
//             }
//         ";
//         test_all(&mut |device, context, queue| {
//             let data: Vec<usize> = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4];
//             let rows = 4;
//             let columns = 3;
//             let dims = Dims::from((rows, columns));
//             let mem_in = DeviceMem::create_read_write_from(context, &data)
//                 .expect("Failed to create_read_write_from data");
//             let mem_result: DeviceMem<usize> = DeviceMem::create_read_write(context, data.len())
//                 .expect("Failed to create_read_write with len");

//             let unbuilt_program = UnbuiltProgram::create_with_source(context, src).unwrap();
//             let devices = vec![device];
//             let programs = unbuilt_program.build(&devices[..]).expect("failed to build_one_on_device");

//             let k = Kernel::create(&programs[0], "transpose_2d")
//                 .expect("failed to create 'transpose_2d' kernel");
//             let () = k
//                 .set_arg(0, &mem_in)
//                 .expect("failed to set mem_in on transpose_2d");
//             let () = k
//                 .set_arg(1, &mem_result)
//                 .expect("failed to set mem_result on transpose_2d");
//             let () = k
//                 .set_arg(2, &rows)
//                 .expect("failed to set rows on transpose_2d");
//             let () = k
//                 .set_arg(3, &columns)
//                 .expect("failed to set columns on transpose_2d");
//             let work = Work::new(dims);
//             assert_eq!(work.n_items(), data.len());
//             let mut output: Vec<usize> = utils::vec_filled_with(0, work.n_items());
//             let _queue_event = queue
//                 .sync_enqueue_kernel(&k, &work)
//                 .expect("failed to sync_enqueue_kernel");
//             let _read_event = queue
//                 .read_buffer(&mem_result, &mut output)
//                 .expect("failed to read transpose_2d mem_out buffer");
//             let expected: Vec<usize> = vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
//             assert_eq!(output, expected);
//         })
//     }
// }
