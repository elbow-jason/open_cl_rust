extern crate open_cl_low_level;

use half::f16;
use open_cl_low_level::*;

fn main() {
    let bytes: Vec<u8> = vec![0, 1, 2, 3];
    println!("bytes {:?}", bytes);
    let ints: Vec<i32> = bytes.try_cl_cast_number().unwrap();
    println!("bytes casted to ints {:?}", ints);
    let floats: Vec<f32> = ints.try_cl_cast_number().unwrap();
    println!("ints casted to floats {:?}", floats);
    let halves: Vec<f16> = floats.try_cl_cast_number().unwrap();
    println!("floats casted to halves {:?}", halves);
    let ulongs: Vec<cl_ulong> = halves.try_cl_cast_number().unwrap();
    println!("halves casted to ulongs {:?}", ulongs);
}
