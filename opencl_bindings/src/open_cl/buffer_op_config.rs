pub struct BufferOpConfig {
    pub is_blocking: bool,
    pub offset: usize,
}

impl Default for BufferOpConfig {
    fn default() -> BufferOpConfig {
        BufferOpConfig {
            is_blocking: true,
            offset: 0,
        }
    }
}
