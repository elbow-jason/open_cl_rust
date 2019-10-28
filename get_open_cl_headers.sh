#!/bin/bash
# Created by Robert Wang.
# From: https://github.com/robertwgh/get-opencl-headers

DIR=$(pwd)/include/

echo "  Downloading OpenCL header files..."

CLDIR="${DIR}OpenCL/"
if [ -d "$CLDIR" ]; then rm -rf $CLDIR; fi
mkdir -p $CLDIR
cd $CLDIR

wget -q --show-progress --directory-prefix=$CLDIR \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_d3d10.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_ext.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_gl.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_gl_ext.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_platform.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/opencl.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_d3d11.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_dx9_media_sharing.h \
        https://raw.githubusercontent.com/KhronosGroup/OpenCL-Headers/master/CL/cl_egl.h 

echo "  Saved OpenCL header files to ${CLDIR}"
