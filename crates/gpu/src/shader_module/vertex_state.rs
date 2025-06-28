use wgpu::{PipelineCompilationOptions, ShaderModule, VertexBufferLayout, VertexState};

pub trait VertexStateExt<'a> {
    fn new(module: &'a ShaderModule, entry_point: &'a str) -> Self;
    fn with_buffers(self, buffers: &'a [VertexBufferLayout<'a>]) -> Self;
    fn with_compilation_options(self, options: PipelineCompilationOptions<'a>) -> Self;
}

impl<'a> VertexStateExt<'a> for VertexState<'a> {
    fn new(module: &'a ShaderModule, entry_point: &'a str) -> Self {
        let entry_point = Some(entry_point);
        Self {
            module,
            entry_point,
            buffers: &[],
            compilation_options: Default::default(),
        }
    }

    fn with_buffers(mut self, buffers: &'a [VertexBufferLayout<'a>]) -> Self {
        self.buffers = buffers;
        self
    }

    fn with_compilation_options(mut self, options: PipelineCompilationOptions<'a>) -> Self {
        self.compilation_options = options;
        self
    }
}
