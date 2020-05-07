// extern crate open_cl_low_level;

// use std::fmt;

// use open_cl_low_level::*;

fn main() {
    //     run_procedural();
    //     run_with_session();
}

// fn run_procedural() {
//     unsafe {
//         let src = include_str!("simple_add.ocl");

//         let mut platforms = list_platforms().unwrap();

//         if platforms.len() == 0 {
//             panic!("No platforms found!!!");
//         }

//         let platform = platforms.remove(0);
//         let devices = list_devices_by_type(&platform, DeviceType::ALL).unwrap();

//         if devices.len() == 0 {
//             panic!("No devices found!!!");
//         }
//         let context = ClContext::create(&devices[..]).unwrap();

//         println!("creating program...");
//         let mut program: ClProgram = ClProgram::create_with_source(&context, src).unwrap();

//         let names = devices.iter().map(|d| d.name().unwrap());
//         println!("building program on devices {:?}...", names);

//         let () = program
//             .build(&devices[..])
//             .unwrap_or_else(|e| panic!("Failed to build program {:?}", e));

//         for device in devices[0..1].iter() {
//             let program2 = (&program).clone();
//             let r_count = program2.reference_count().unwrap();
//             let prog_log = program2.get_log(device).unwrap();
//             let prog_src = program2.source().unwrap();
//             println!("Program log {:?} {:?}, {:?}", r_count, prog_log, prog_src);
//             println!("Device {:?}", device);

//             let mut command_queue: ClCommandQueue =
//                 ClCommandQueue::create(&context, device, None).unwrap();

//             let vec_a = vec![1i64, 2, 3];
//             let vec_b = vec![0i64, -1, -2];

//             let len = vec_a.len();

//             let work: Work = Work::new(len);
//             let name = device.name().unwrap();
//             println!("{}", name);

//             let mut mem_a = ClMem::create::<i64, usize>(
//                 &context,
//                 len,
//                 HostAccess::WriteOnly,
//                 KernelAccess::ReadOnly,
//                 MemLocation::AllocOnDevice,
//             )
//             .unwrap();
//             let mut mem_b = ClMem::create::<i64, usize>(
//                 &context,
//                 len,
//                 HostAccess::WriteOnly,
//                 KernelAccess::ReadOnly,
//                 MemLocation::AllocOnDevice,
//             )
//             .unwrap();
//             let mut mem_c = ClMem::create::<i64, usize>(
//                 &context,
//                 len,
//                 HostAccess::ReadOnly,
//                 KernelAccess::WriteOnly,
//                 MemLocation::AllocOnDevice,
//             )
//             .unwrap();
//             println!("Creating kernel simple_add");
//             let mut simple_add = ClKernel::create(&program2, "simple_add").unwrap();

//             println!("writing buffer a...");
//             let _write_event_a = command_queue
//                 .write_buffer(&mut mem_a, &vec_a[..], None)
//                 .unwrap();

//             println!("writing buffer b...");
//             let _write_event_b = command_queue
//                 .write_buffer(&mut mem_b, &vec_b[..], None)
//                 .unwrap();

//             println!("mem_a {:?}", mem_a);

//             println!("setting simple_add arg 0 as mem_a");
//             simple_add.set_arg(0, &mut mem_a).unwrap();

//             println!("setting simple_add arg 1 as mem_b");
//             simple_add.set_arg(1, &mut mem_b).unwrap();

//             println!("setting simple_add mut arg 2 as mem_c");
//             simple_add.set_arg(2, &mut mem_c).unwrap();

//             println!("calling enqueue_kernel on simple_add");
//             let event = command_queue
//                 .enqueue_kernel(&mut simple_add, &work, None)
//                 .unwrap();
//             let () = event.wait().unwrap();
//             println!("done putting event into WaitList...");
//             let mut vec_c = vec![0i64; len];

//             let _read_event = command_queue
//                 .read_buffer(&mem_c, &mut vec_c[..], None)
//                 .unwrap();

//             println!("  {}", string_from_slice(&vec_a[..]));
//             println!("+ {}", string_from_slice(&vec_b[..]));
//             println!("= {}", string_from_slice(&vec_c[..]));
//         }
//     }
// }

// fn run_with_session() {
//     let src = include_str!("simple_add.ocl");
//     unsafe {
//         let mut session = SessionBuilder::new().with_program_src(src).build().unwrap();

//         let vec_a = vec![1i64, 2, 3];
//         let vec_b = vec![0i64, -1, -2];

//         let mut mem_a = session.create_mem(&vec_a[..]).unwrap();
//         let mut mem_b = session.create_mem(&vec_b[..]).unwrap();
//         let mut mem_c: ClMem = session.create_mem::<i64, usize>(vec_a.len()).unwrap();

//         let mut simple_add = session.create_kernel("simple_add").unwrap();

//         simple_add.set_arg(0, &mut mem_a).unwrap();
//         simple_add.set_arg(1, &mut mem_b).unwrap();
//         simple_add.set_arg(2, &mut mem_c).unwrap();
//         let work: Work = Work::new(vec_a.len());

//         let mut vec_c = vec_a.clone();

//         let enqueue_event = session
//             .enqueue_kernel(0, &mut simple_add, &work, None)
//             .unwrap();
//         let () = enqueue_event.wait().unwrap();
//         let mut read_event = session
//             .read_buffer(0, &mut mem_c, &mut vec_c[..], None)
//             .unwrap();
//         let read_output = read_event.wait().unwrap();
//         assert_eq!(read_output, None);

//         // at this point vec_c *should* be the result of calling simple_add and reading from mem_c;
//         println!("  {}", string_from_slice(&vec_a[..]));
//         println!("+ {}", string_from_slice(&vec_b[..]));
//         println!("= {}", string_from_slice(&vec_c[..]));
//     }
// }

// fn string_from_slice<T: fmt::Display>(slice: &[T]) -> String {
//     let mut st = String::from("[");
//     let mut first = true;

//     for i in slice.iter() {
//         if !first {
//             st.push_str(", ");
//         } else {
//             first = false;
//         }
//         st.push_str(&*i.to_string())
//     }

//     st.push_str("]");
//     return st;
// }
