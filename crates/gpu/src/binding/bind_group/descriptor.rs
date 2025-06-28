use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout};

pub trait BindGroupDescriptorExt<'a> {
    fn new(layout: &'a BindGroupLayout, entries: &'a [BindGroupEntry]) -> Self;
}

impl<'a> BindGroupDescriptorExt<'a> for BindGroupDescriptor<'a> {
    fn new(layout: &'a BindGroupLayout, entries: &'a [BindGroupEntry]) -> Self {
        Self { layout, entries, label: None }
    }
}

crate::impl_label!(BindGroupDescriptor<'a>);