#if defined(__APPLE__) || defined(__MACOSX)
// #include <OpenCL.cl.h>
#define CL_TARGET_OPENCL_VERSION 110
#include "include/CL/cl.h"
#else
#include <CL/cl.h>

#endif