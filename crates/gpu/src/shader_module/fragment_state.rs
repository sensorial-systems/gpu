use wgpu::{ColorTargetState, PipelineCompilationOptions, ShaderModule, FragmentState};

pub trait FragmentStateExt<'a> {
    fn new(module: &'a ShaderModule, entry_point: &'a str) -> Self;
    fn with_compilation_options(self, options: PipelineCompilationOptions<'a>) -> Self;
    fn with_targets(self, targets: &'a [Option<ColorTargetState>]) -> Self;
}

impl<'a> FragmentStateExt<'a> for FragmentState<'a> {
    fn new(module: &'a ShaderModule, entry_point: &'a str) -> Self {
        let entry_point = Some(entry_point);
        Self {
            module,
            entry_point,
            targets: Default::default(),
            compilation_options: Default::default(),
        }
    }

    fn with_compilation_options(mut self, options: PipelineCompilationOptions<'a>) -> Self {
        self.compilation_options = options;
        self
    }

    fn with_targets(mut self, targets: &'a [Option<ColorTargetState>]) -> Self {
        self.targets = targets;
        self
    }
}
