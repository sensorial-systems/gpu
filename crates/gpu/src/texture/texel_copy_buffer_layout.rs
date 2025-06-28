use wgpu::TexelCopyBufferLayout;

pub trait TexelCopyBufferLayoutExt {
    fn new(bytes_per_row: Option<u32>, rows_per_image: Option<u32>) -> Self;
    fn with_offset(self, offset: u64) -> Self;
}

impl TexelCopyBufferLayoutExt for TexelCopyBufferLayout {
    fn new(bytes_per_row: Option<u32>, rows_per_image: Option<u32>) -> Self {
        TexelCopyBufferLayout { offset: 0, bytes_per_row, rows_per_image }
    }

    fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }
}