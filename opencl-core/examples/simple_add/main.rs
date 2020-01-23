extern crate opencl_core;

use std::fmt;

use opencl_core::{
    Platform, Device, Context, UnbuiltProgram, Program, CommandQueue, Buffer,
    Kernel,
};
use opencl_core::ll::{
    DevicePtr, ProgramPtr, HostAccess, KernelAccess, MemLocation, Work,
    SessionBuilder, ClMem, Waitlist, MutHostBuffer, HostBuffer,
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
    let unbuilt_program: UnbuiltProgram = UnbuiltProgram::create_with_source(&context, src).unwrap();

    let names = devices.iter().map(|d| d.name().unwrap());
    println!("building program on devices {:?}...", names);

    let program: Program = unbuilt_program.build(&devices[..]).unwrap_or_else(|e| panic!("Failed to build program {:?}", e));
    
    for device in devices[0..1].iter() {  
        let program2 = (&program).clone();
        let r_count = program2.reference_count().unwrap();
        let prog_log = program2.low_level_program().get_log(device).unwrap();
        let prog_src = program2.low_level_program().source().unwrap();
        println!("Program log {:?} {:?}, {:?}", r_count, prog_log, prog_src);
        println!("Device {:?}", device);


        let command_queue: CommandQueue = CommandQueue::create(&context, device, None).unwrap();

        let vec_a = vec![1isize, 2, 3];
        let vec_b = vec![0isize, -1, -2];

        let len = vec_a.len();

        let work: Work = Work::new(len);
        let name = device.name().unwrap();
        println!("{}", name);

        let mem_a = Buffer::create(&context, len, HostAccess::WriteOnly, KernelAccess::ReadOnly, MemLocation::AllocOnDevice).unwrap();
        let mem_b = Buffer::create(&context, len, HostAccess::WriteOnly, KernelAccess::ReadOnly, MemLocation::AllocOnDevice).unwrap();
        let mem_c = Buffer::create(&context, len, HostAccess::ReadOnly, KernelAccess::WriteOnly, MemLocation::AllocOnDevice).unwrap();
        println!("Creating kernel simple_add");
        let simple_add = Kernel::create(&program2, "simple_add").unwrap();

        println!("writing buffer a...");
        let _write_event_a = command_queue.write_buffer(&mem_a, &vec_a, None).unwrap();

        println!("writing buffer b...");
        let _write_event_b = command_queue.write_buffer(&mem_b, &vec_b, None).unwrap();

        println!("mem_a {:?}", mem_a);

        let lock_a = mem_a.write_lock();
        println!("setting simple_add arg 0 as mem_a");
        simple_add.set_arg(0, &*lock_a).unwrap();

        let lock_b = mem_b.write_lock();
        println!("setting simple_add arg 1 as mem_b");
        simple_add.set_arg(1, &*lock_b).unwrap();

        let lock_c = mem_c.write_lock();
        println!("setting simple_add mut arg 2 as mem_c");
        simple_add.set_arg(2, &*lock_c).unwrap();

        println!("calling enqueue_kernel on simple_add");
        let () = command_queue.enqueue_kernel(&simple_add, &work, None).unwrap();

        println!("Dropping locks...");
        std::mem::drop(lock_a);
        std::mem::drop(lock_b);
        std::mem::drop(lock_c);

        println!("done putting event into WaitList...");
        let mut vec_c: Vec<isize> = vec![0; len];

        let _read_event = command_queue.read_buffer(&mem_c, &mut vec_c, None).unwrap();

        println!("  {}", string_from_slice(&vec_a[..]));
        println!("+ {}", string_from_slice(&vec_b[..]));
        println!("= {}", string_from_slice(&vec_c[..]));
    }
}

fn run_with_session() {
    let src = include_str!("simple_add.ocl");
     unsafe {
        let session = SessionBuilder::new().with_program_src(src).build().unwrap();
        
        let vec_a = vec![1isize, 2, 3];
        let vec_b = vec![0isize, -1, -2];

        let mem_a = session.create_mem(&vec_a[..]).unwrap();
        let mem_b = session.create_mem(&vec_b[..]).unwrap();
        let mem_c: ClMem<isize> = session.create_mem(vec_a.len()).unwrap();
        
        let mut simple_add = session.create_kernel("simple_add").unwrap();

        println!("setting simple_add arg 0 as mem_a");
        simple_add.set_arg(0, &mem_a).unwrap();

        
        println!("setting simple_add arg 1 as mem_b");
        simple_add.set_arg(1, &mem_b).unwrap();

        
        println!("setting simple_add mut arg 2 as mem_c");
        simple_add.set_arg(2, &mem_c).unwrap();
        let work: Work = Work::new(vec_a.len());
        
        let mut vec_c = vec_a.clone();

    

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
