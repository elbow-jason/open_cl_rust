# open_cl_rust

A safe, fast, no nonsense Rust lib for interacting with OpenCL.

NOTE: As of right now this library is very alpha software. Use at your own risk!


## C FFI Philosophy

  + All interaction with a raw pointer is unsafe.
    
  + All functions that take a raw pointer as an arg are unsafe.

  + All functions that return a raw pointer are unsafe.

  + A function that is not marked `unsafe` cannot return a raw pointer.

  + Only allow access to raw pointers via functions marked as `unsafe`.

## Learning Resources

 + https://www.khronos.org/registry/OpenCL/sdk/1.2/docs/man/xhtml/

 + http://www.aronaldg.org/webfiles/compecon/src/opencl/doc/OpenCL_Mac_OS_X.pdf

## Notes 

  + NOTE: OpenCL 1.0 is not thread safe. We will not support it.
  
  + NOTE: Investigate OpenCL restrictions around numbers and safety of `num` crate for OpenCL.
  
  + NOTE: Implement Sampler.
  
  + NOTE: Implement UserEvent for (clCreateUserEvent, clSetUserEventStatus)
  
  + NOTE: Implement Markers
  
  + NOTE: Implement Barriers