use crate::prelude::*;

use wgpu::{BindGroupEntry, BindingResource};

pub trait BindGroupEntryExt<'a> {
    fn new(binding: u32, resource: impl IntoExt<BindingResource<'a>>) -> Self;
}

impl<'a> BindGroupEntryExt<'a> for BindGroupEntry<'a> {
    fn new(binding: u32, resource: impl IntoExt<BindingResource<'a>>) -> Self {
        let resource = resource.into_ext();
        BindGroupEntry { binding, resource }
    }
}
