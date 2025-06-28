mod descriptor;
pub use descriptor::*;

use crate::prelude::*;
use wgpu::{CommandEncoder, CommandEncoderDescriptor, Device};

pub trait CommandEncoderExt {
    fn new<'a>(device: &Device, descriptor: impl IntoExt<CommandEncoderDescriptor<'a>>) -> Self;
}

impl CommandEncoderExt for CommandEncoder {
    fn new<'a>(device: &Device, descriptor: impl IntoExt<CommandEncoderDescriptor<'a>>) -> Self {
        device.create_command_encoder(&descriptor.into_ext())
    }
}