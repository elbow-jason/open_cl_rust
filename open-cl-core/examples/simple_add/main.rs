extern crate open_cl_core;

use std::fmt;

use open_cl_core::ll::{DevicePtr, ProgramPtr};
use open_cl_core::{
    Buffer, CommandQueue, Context, Device, Kernel, Platform, Program, UnbuiltProgram, Work,
};

fn main() {
    run_procedural()
}

fn run_procedural() {
    let src = include_str!("simple_add.ocl");

    let mut platforms = Platform::list_all().unwrap();

    if platforms.len() == 0 {
        panic!("No platforms found!!!");
    }

    let platform = platforms.remove(0);
    let devices = Device::list_all_devices(&platform).unwrap();

    if devices.len() == 0 {
        panic!("No devices found!!!");
    }
    let context = Context::create(&devices[..]).unwrap();

    println!("creating program...");
    let unbuilt_program: UnbuiltProgram =
        UnbuiltProgram::create_with_source(&context, src).unwrap();

    let names = devices.iter().map(|d| d.low_level_device().name().unwrap());
    println!("building program on devices {:?}...", names);

    let program: Program = unbuilt_program
        .build(&devices[..])
        .unwrap_or_else(|e| panic!("Failed to build program {:?}", e));

    for device in devices[0..1].iter() {
        let program2 = (&program).clone();
        let r_count = program2.low_level_program().reference_count().unwrap();
        let prog_log = program2.low_level_program().get_log(device).unwrap();
        let prog_src = program2.low_level_program().source().unwrap();
        println!("Program log {:?} {:?}, {:?}", r_count, prog_log, prog_src);
        println!("Device {:?}", device);

        let command_queue: CommandQueue = CommandQueue::create(&context, device, None).unwrap();

        let vec_a = vec![1i64, 2, 3];
        let vec_b = vec![0i64, -1, -2];

        let len = vec_a.len();

        let work: Work = Work::new(len);
        let name = device.name().unwrap();
        println!("{}", name);

        let mem_a = Buffer::create_with_len::<i64>(&context, len).unwrap();
        let mem_b = Buffer::create_with_len::<i64>(&context, len).unwrap();
        let mem_c = Buffer::create_with_len::<i64>(&context, len).unwrap();
    
        println!("Creating kernel simple_add");
        let simple_add = Kernel::create(&program2, "simple_add").unwrap();

        println!("writing buffer a...");
        let _write_event_a = command_queue.write_buffer(&mem_a, &vec_a, None).unwrap();

        println!("writing buffer b...");
        let _write_event_b = command_queue.write_buffer(&mem_b, &vec_b, None).unwrap();

        println!("mem_a {:?}", mem_a);

        let mut lock_a = mem_a.write_lock();
        println!("setting simple_add arg 0 as mem_a");
        unsafe { simple_add.set_arg(0, &mut *lock_a).unwrap() };

        let mut lock_b = mem_b.write_lock();
        println!("setting simple_add arg 1 as mem_b");
        unsafe { simple_add.set_arg(1, &mut *lock_b).unwrap() };

        let mut lock_c = mem_c.write_lock();
        println!("setting simple_add mut arg 2 as mem_c");
        unsafe { simple_add.set_arg(2, &mut *lock_c).unwrap() };

        println!("calling enqueue_kernel on simple_add");
        let () = command_queue
            .enqueue_kernel(simple_add, &work, None)
            .unwrap();

        println!("Dropping locks...");
        std::mem::drop(lock_a);
        std::mem::drop(lock_b);
        std::mem::drop(lock_c);

        println!("done putting event into WaitList...");
        let mut vec_c: Vec<i64> = vec![0; len];

        let _read_event = command_queue.read_buffer(&mem_c, &mut vec_c, None).unwrap();

        println!("  {}", string_from_slice(&vec_a[..]));
        println!("+ {}", string_from_slice(&vec_b[..]));
        println!("= {}", string_from_slice(&vec_c[..]));
    }
}

fn string_from_slice<T: fmt::Display>(slice: &[T]) -> String {
    let mut st = String::from("[");
    let mut first = true;

    for i in slice.iter() {
        if !first {
            st.push_str(", ");
        } else {
            first = false;
        }
        st.push_str(&*i.to_string())
    }

    st.push_str("]");
    return st;
}
