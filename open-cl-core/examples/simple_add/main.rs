extern crate open_cl_core;
use open_cl_core::{HasDeviceInfo, KernelOperation, Platform, Session, Work};
use std::fmt;

fn main() {
    run_procedural()
}

fn run_procedural() {
    let src = include_str!("simple_add.ocl");

    let platforms = Platform::list_all().unwrap();

    if platforms.len() == 0 {
        panic!("No platforms found!!!");
    }
    for platform in platforms.into_iter() {
        let devices = platform.list_all_devices().unwrap();

        if devices.len() == 0 {
            panic!("No devices found!!!");
        }
        for device in devices.into_iter() {
            println!(
                "running simple add on device {:?} ...",
                device.low_level_device().name().unwrap()
            );
            let sessions = Session::create_with_devices(vec![device], src, None).unwrap();
            for session in sessions.into_iter() {
                let vec_a = vec![1i64, 2, 3];
                let vec_b = vec![0i64, -1, -2];
                let len = vec_a.len();
                let work: Work = Work::new(len);
                let buffer_a = session.create_buffer::<i64, usize>(len).unwrap();
                let buffer_b = session.create_buffer::<i64, usize>(len).unwrap();
                let buffer_c = session.create_buffer::<i64, usize>(len).unwrap();
                println!("writing buffer a...");
                let _write_event_a = session
                    .sync_write_buffer(&buffer_a, &vec_a[..], None)
                    .unwrap();
                println!("writing buffer b...");
                let _write_event_b = session
                    .sync_write_buffer(&buffer_b, &vec_b[..], None)
                    .unwrap();

                let simple_add = KernelOperation::new("simple_add")
                    .with_work(work)
                    .add_arg(&buffer_a)
                    .add_arg(&buffer_b)
                    .add_arg(&buffer_c);
                println!("executing simple_add: c[i] = a[i] + b[i]");
                session.execute_sync_kernel_operation(simple_add).unwrap();
                let mut vec_c: Vec<i64> = vec![0; len];
                let _read_event = session
                    .sync_read_buffer(&buffer_c, &mut vec_c[..], None)
                    .unwrap();
                println!("  {}", string_from_slice(&vec_a[..]));
                println!("+ {}", string_from_slice(&vec_b[..]));
                println!("= {}", string_from_slice(&vec_c[..]));
            }
        }
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
