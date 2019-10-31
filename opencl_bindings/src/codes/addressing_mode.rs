/* cl_addressing_mode */
use ffi::cl_addressing_mode;

crate::__codes_enum!(AddressingMode, cl_addressing_mode, {
    NoneType => 0x1130,
    ClampToEdge => 0x1131,
    Clamp => 0x1132,
    Repeat => 0x1133,
    MirroredRepeat => 0x1134
});
