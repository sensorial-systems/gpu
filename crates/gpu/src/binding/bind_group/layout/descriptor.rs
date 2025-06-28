use wgpu::{BindGroupLayoutDescriptor, BindGroupLayoutEntry};

pub trait BindGroupLayoutDescriptorExt<'a> {
    fn new(entries: &'a [BindGroupLayoutEntry]) -> Self;
}

impl<'a> BindGroupLayoutDescriptorExt<'a> for BindGroupLayoutDescriptor<'a> {
    fn new(entries: &'a [BindGroupLayoutEntry]) -> Self {
        Self { entries, label: None }
    }
}

crate::impl_label!(BindGroupLayoutDescriptor<'a>);