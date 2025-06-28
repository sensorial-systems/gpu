use crate::prelude::*;

use wgpu::CommandEncoderDescriptor;

pub trait CommandEncoderDescriptorExt<'a> {
    fn new() -> Self;
}

impl<'a> FromExt<&'a str> for CommandEncoderDescriptor<'a> {
    fn from_ext(value: &'a str) -> Self {
        CommandEncoderDescriptor { label: Some(value) }
    }
}

impl<'a> CommandEncoderDescriptorExt<'a> for CommandEncoderDescriptor<'a> {
    fn new() -> Self {
        Default::default()
    }
}

crate::impl_label!(CommandEncoderDescriptor<'a>);