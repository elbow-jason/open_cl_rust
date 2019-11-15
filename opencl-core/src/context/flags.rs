use crate::ffi::{cl_context_info, cl_context_properties};

crate::__codes_enum!(ContextInfo, cl_context_info, {
    ReferenceCount => 0x1080,
    Devices => 0x1081,
    Properties => 0x1082,
    NumDevices => 0x1083
});

crate::__codes_enum!(ContextProperties, cl_context_properties, {
    Platform => 0x1084,
    InteropUserSync => 0x1085
});

// NOTE: Add support for d3d and khr flags
// https://www.khronos.org/registry/OpenCL/sdk/1.2/docs/man/xhtml/clCreateContext.html
// CL_CONTEXT_D3D10_DEVICE_KHR 	    ID3D10Device* 	                    default: NULL
// CL_GL_CONTEXT_KHR 	            0, OpenGL context handle 	        (available if the cl_khr_gl_sharing extension is enabled)
// CL_EGL_DISPLAY_KHR 	            EGL_NO_DISPLAY, EGLDisplay handle 	(available if the cl_khr_gl_sharing extension is enabled)
// CL_GLX_DISPLAY_KHR 	            None, X handle 	                    (available if the cl_khr_gl_sharing extension is enabled)
// CL_CGL_SHAREGROUP_KHR 	        0, CGL share group handle 	        (available if the cl_khr_gl_sharing extension is enabled)
// CL_WGL_HDC_KHR 	                0, HDC handle 	                    (available if the cl_khr_gl_sharing extension is enabled)
// CL_CONTEXT_ADAPTER_D3D9_KHR      IDirect3DDevice9 *                  (if the cl_khr_dx9_media_sharing extension is supported).
// CL_CONTEXT_ADAPTER_D3D9EX_KHR    IDirect3DDeviceEx*                  (if the cl_khr_dx9_media_sharing extension is supported).
// CL_CONTEXT_ADAPTER_DXVA_KHR      IDXVAHD_Device *                    (if the cl_khr_dx9_media_sharing extension is supported).
// CL_CONTEXT_D3D11_DEVICE_KHR      ID3D11Device *                      default: NULL
