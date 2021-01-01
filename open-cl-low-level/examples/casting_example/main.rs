extern crate open_cl_low_level;

use open_cl_low_level::*;

fn main() {
    let bytes: Vec<u8> = vec![0, 1, 2, 3];
    println!("bytes {:?}", bytes);
    let ints: Vec<i32> = NumCastFrom::num_cast_from(bytes).unwrap();
    println!("bytes casted to ints {:?}", ints);
    let floats: Vec<f32> = NumCastFrom::num_cast_from(ints).unwrap();
    println!("ints casted to floats {:?}", floats);
    // let halves: Vec<F16> = NumCastFrom::num_cast_from(floats).unwrap();
    // println!("floats casted to halves {:?}", halves);
    let ulongs: Vec<u64> = NumCastFrom::num_cast_from(floats).unwrap();
    println!("floats casted to u64s {:?}", ulongs);
}
