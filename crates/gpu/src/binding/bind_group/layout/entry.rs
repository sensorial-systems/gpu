use std::num::NonZeroU32;

use wgpu::{BindGroupLayoutEntry, BindingType, ShaderStages};
pub trait BindGroupLayoutEntryExt {
    fn new(binding: u32, visibility: ShaderStages, ty: BindingType) -> Self;
    fn with_count(self, count: Option<NonZeroU32>) -> Self;
}

impl BindGroupLayoutEntryExt for BindGroupLayoutEntry {
    fn new(binding: u32, visibility: ShaderStages, ty: BindingType) -> Self {
        BindGroupLayoutEntry { binding, visibility, ty, count: None }
    }

    fn with_count(mut self, count: Option<NonZeroU32>) -> Self {
        self.count = count;
        self
    }
}