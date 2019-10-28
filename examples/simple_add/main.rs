extern crate open_cl_bindings;

use std::fmt;

use open_cl_bindings::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => panic!("Example simple_add.rs failed: {:?}", e),
    }
}

fn run() -> Result<(), Error> {

    // clSetKernelArg(kernel, 0, sizeof(my_buffer), &my_buffer);
    // int my_int = 50;
    // clSetKernelArg(kernel, 1, sizeof(my_int), &my_int);
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

    let vec_a = vec![0isize, 1, 2, -3, 4, 5, 6, 7, 8];
    let vec_b = vec![-7isize, -6, 5, -4, 0, -1, 2, 3, 9];

    let buffer_size = vec_a.len();

    let work: Work = Work::new(buffer_size);

    println!("{}", name);

    let mem_a: KernelReadWriteMem<isize> = context.create_read_write_buffer(buffer_size)?;
    let mem_b: KernelReadWriteMem<isize> = context.create_read_write_buffer(buffer_size)?;
    let mem_c: KernelReadWriteMem<isize> = context.create_read_write_buffer(buffer_size)?;

    let buffer_a: HostBuffer<isize> = HostBuffer::new(vec_a);
    let buffer_b: HostBuffer<isize> = HostBuffer::new(vec_b);
    
    println!("fetching_kernel simple_add");
    let simple_add: Kernel = program.fetch_kernel("simple_add")?;
    

    // let my_mem_a: cl_mem = mem_a.cl_mem();
    // let my_ctx: cl_context = context.cl_object();
    // let my_mem_flag = CL_MEM_READ_WRITE;
    // let my_mem_size = std::mem::size_of::<cl_mem>() as libc::size_t;
    // let my_kernel: cl_kernel = simple_add.cl_object();
    // let out = unsafe { clSetKernelArg(my_kernel, 0, my_mem_size, my_mem_a as *const libc::c_void) };
    // assert!(out == 0);
    println!("writing buffer a...");
    let () = command_queue.write_buffer(&mem_a, &buffer_a, EventList::empty(), None)?;

    println!("writing buffer b...");
    let () = command_queue.write_buffer(&mem_b, &buffer_b, EventList::empty(), None)?;

    println!("mem_a ptr {:?}", mem_a.cl_mem());

    println!("setting simple_add arg 0 as mem_a");
    simple_add.set_arg::<&KernelReadWriteMem<isize>, cl_mem>(0, &mem_a)?;

    println!("setting simple_add arg 1 as mem_b");
    simple_add.set_arg::<&KernelReadWriteMem<isize>, cl_mem>(1, &mem_b)?;

    println!("setting simple_add mut arg 2 as mem_c");
    simple_add.set_arg::<&KernelReadWriteMem<isize>, cl_mem>(2, &mem_c)?;

    println!("calling sync_enqueue_kernel on simple_add");
    let event = command_queue.sync_enqueue_kernel(simple_add, work, EventList::empty())?;
    println!("putting event into EventList...");
    let event_list: EventList = EventList::from(event);
    println!("done putting event into EventList...");
    let vec_c: Vec<isize> = vec![0; buffer_size];
    let mut buffer_c: HostBuffer<isize> = HostBuffer::new(vec_c);


    let () = command_queue.read_buffer(&mem_c, &mut buffer_c, event_list, None)?;

    let vec_a2 = buffer_a.into_data();
    let vec_b2 = buffer_b.into_data();
    let vec_c2 = buffer_c.into_data();

    println!("  {}", string_from_slice(&vec_a2[..]));
    println!("+ {}", string_from_slice(&vec_b2[..]));
    println!("= {}", string_from_slice(&vec_c2[..]));
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
