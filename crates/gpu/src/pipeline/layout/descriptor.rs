use wgpu::{BindGroupLayout, PipelineLayoutDescriptor, PushConstantRange};

pub trait PipelineLayoutDescriptorExt<'label>: Sized {
    fn new() -> Self;
    fn with_bind_group_layouts(self, bind_group_layouts: &'label [&'label BindGroupLayout]) -> Self;
    fn with_push_constant_ranges(self, push_constant_ranges: &'label [PushConstantRange]) -> Self;
}

impl<'label> PipelineLayoutDescriptorExt<'label> for PipelineLayoutDescriptor<'label> {
    fn new() -> Self {
        Default::default()
    }

    fn with_bind_group_layouts(mut self, bind_group_layouts: &'label [&'label BindGroupLayout]) -> Self {
        self.bind_group_layouts = bind_group_layouts;
        self
    }
    
    fn with_push_constant_ranges(mut self, push_constant_ranges: &'label [PushConstantRange]) -> Self {
        self.push_constant_ranges = push_constant_ranges;
        self
    }
}

crate::impl_label!(PipelineLayoutDescriptor<'label>);