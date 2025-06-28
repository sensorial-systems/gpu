use wgpu::{util::BufferInitDescriptor, BufferUsages};

pub trait BufferInitDescriptorExt<'label> {
    fn new<A: bytemuck::NoUninit>(contents: &'label [A], usage: BufferUsages) -> Self;
}

impl<'label> BufferInitDescriptorExt<'label> for BufferInitDescriptor<'label> {    
    fn new<A: bytemuck::NoUninit>(contents: &'label [A], usage: BufferUsages) -> Self {
        Self {
            label: Default::default(),
            contents: bytemuck::cast_slice(contents),
            usage,
        }
    }
}

crate::impl_label!(BufferInitDescriptor<'label>);