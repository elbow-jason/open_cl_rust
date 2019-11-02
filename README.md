http://www.aronaldg.org/webfiles/compecon/src/opencl/doc/OpenCL_Mac_OS_X.pdf


### Mac SDK
    
    `cd /Library/Developer/CommandLineTools/Packages`
    `open macOS_SDK_headers_for_macOS_10.14.pkg`


NOTE: OpenCL 1.0 is not thread safe. We will not support it.

NOTE: All interaction with raw pointer is unsafe.
NOTE: All functions that take a raw pointer as an arg are unsafe.
NOTE: All functions that return a raw pointer are unsafe.



NOTE: Investigate OpenCL restrictions around numbers and safety for num crate