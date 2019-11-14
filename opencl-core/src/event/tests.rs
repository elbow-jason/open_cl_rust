// #[test]
//         fn event_get_times_test() {
//             let src = "__kernel void test(__global int *i) { \
//                        *i += 1; \
//                        }";

//             let (device, ctx, queue) = util::create_compute_context().unwrap();
//             let prog = ctx.create_program_from_source(src);
//             prog.build(&device).unwrap();

//             let k = prog.create_kernel("test");
//             let v = ctx.create_buffer_from(vec![1isize], CL_MEM_READ_WRITE);

//             k.set_arg(0, &v);

//             let e = queue.enqueue_async_kernel(&ctx, &k, 1isize, None, ());
//             e.wait();

//             // the that are returned are not useful for unit test, this test
//             // is mostly testing that opencl returns no error
//             e.queue_time();
//             e.submit_time();
//             e.start_time();
//             e.end_time();
//         }