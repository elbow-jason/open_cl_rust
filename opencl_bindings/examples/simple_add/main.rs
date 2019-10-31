extern crate opencl_bindings;

use std::fmt;

use opencl_bindings::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => panic!("Example simple_add.rs failed: {:?}", e),
    }
}

fn run() -> Result<(), Error> {
    let src: String = include_str!("program.ocl").to_string();
    let mut platforms = Platform::all()?;

    if platforms.len() == 0 {
        panic!("No platforms found!!!");
    }

    let platform = platforms.remove(0);
    let mut devices = platform.all_devices()?;

    println!("all_devices {:?}", devices);
    if devices.len() == 0 {
        panic!("No devices found!!!");
    }
    let device = devices.remove(1);
    let context = device.create_context()?;
    let command_queue: CommandQueue = context.create_command_queue(&device, None)?;
    let name = device.name_info()?;

    println!("creating program...");
    let program: Program = context.create_program_with_source(src)?;

    println!("building program on device {}...", name);
    let () = program.build_on_one_device(&device)?;

    let vec_a = vec![1isize, 2, 3];
    let vec_b = vec![0isize, -1, -2];

    let buffer_size = vec_a.len();

    let work: Work = Work::new(buffer_size);

    println!("{}", name);

    let mem_a: DeviceMem<isize> = context.create_read_write_buffer(buffer_size)?;
    let mem_b: DeviceMem<isize> = context.create_read_write_buffer(buffer_size)?;
    let mem_c: DeviceMem<isize> = context.create_read_write_buffer(buffer_size)?;

    println!("fetching_kernel simple_add");
    let simple_add: Kernel = program.fetch_kernel("simple_add")?;
    
    println!("writing buffer a...");
    let _write_event_a = command_queue.write_buffer(&mem_a, &vec_a, WaitList::empty(), None)?;

    println!("writing buffer b...");
    let _write_event_b = command_queue.write_buffer(&mem_b, &vec_b, WaitList::empty(), None)?;

    println!("mem_a {:?}", mem_a);

    println!("setting simple_add arg 0 as mem_a");
    simple_add.set_arg::<DeviceMem<isize>>(0, &mem_a)?;

    println!("setting simple_add arg 1 as mem_b");
    simple_add.set_arg::<DeviceMem<isize>>(1, &mem_b)?;

    println!("setting simple_add mut arg 2 as mem_c");
    simple_add.set_arg::<DeviceMem<isize>>(2, &mem_c)?;

    println!("calling sync_enqueue_kernel on simple_add");
    let _exec_event = command_queue.sync_enqueue_kernel(&simple_add, work, WaitList::empty())?;
    // println!("putting event into WaitList...");
    // let event_list: WaitList = WaitList::from(event);
    println!("done putting event into WaitList...");
    let mut vec_c: Vec<isize> = vec![0; buffer_size];

    let _read_event = command_queue.read_buffer(&mem_c, &mut vec_c, WaitList::empty(), None)?;

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
