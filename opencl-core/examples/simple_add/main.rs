extern crate opencl_core;

use std::fmt;

use opencl_core::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => panic!("Example simple_add.rs failed: {:?}", e),
    }
}

fn run() -> Result<(), Error> {
    let src: &str = include_str!("program.ocl");
    let mut platforms = Platform::all()?;

    if platforms.len() == 0 {
        panic!("No platforms found!!!");
    }

    let platform = platforms.remove(0);
    let mut devices = platform.all_devices()?;

    if devices.len() == 0 {
        panic!("No devices found!!!");
    }
    let device = devices.remove(0);
    let context = Context::create(&[&device])?;

    let command_queue: CommandQueue = CommandQueue::create(&context, &device, None)?;
    let name = device.name()?;

    println!("creating program...");
    let unbuilt_program: UnbuiltProgram = UnbuiltProgram::create_with_source(&context, src)?;

    println!("building program on device {}...", name);
    let mut programs: Vec<Program> = unbuilt_program.build(&[device])?;
    assert_eq!(programs.len(), 1);
    let program = programs.remove(0);
    
    let vec_a = vec![1isize, 2, 3];
    let vec_b = vec![0isize, -1, -2];

    let len = vec_a.len();

    let work: Work = Work::new(len);

    println!("{}", name);

    let mem_a = DeviceMem::create_read_write(&context, len)?;
    let mem_b = DeviceMem::create_read_write(&context, len)?;
    let mem_c = DeviceMem::create_read_write(&context, len)?;
    &println!("fetching_kernel simple_add");
    let simple_add = Kernel::create(&program, "simple_add")?;

    println!("writing buffer a...");
    let _write_event_a = command_queue.write_buffer(&mem_a, &vec_a)?;

    println!("writing buffer b...");
    let _write_event_b = command_queue.write_buffer(&mem_b, &vec_b)?;

    println!("mem_a {:?}", mem_a);

    println!("setting simple_add arg 0 as mem_a");
    simple_add.set_arg(0, &mem_a)?;

    println!("setting simple_add arg 1 as mem_b");
    simple_add.set_arg(1, &mem_b)?;

    println!("setting simple_add mut arg 2 as mem_c");
    simple_add.set_arg(2, &mem_c)?;

    println!("calling sync_enqueue_kernel on simple_add");
    let _exec_event = command_queue.sync_enqueue_kernel(&simple_add, &work)?;

    println!("done putting event into WaitList...");
    let mut vec_c: Vec<isize> = vec![0; len];

    let _read_event = command_queue.read_buffer(&mem_c, &mut vec_c)?;

    println!("  {}", string_from_slice(&vec_a[..]));
    println!("+ {}", string_from_slice(&vec_b[..]));
    println!("= {}", string_from_slice(&vec_c[..]));
    Ok(())
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
