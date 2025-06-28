use crate::prelude::*;

use wgpu::{PowerPreference, RequestAdapterOptions, Surface};

pub trait RequestAdapterOptionsExt<'surface, 'window> {
    fn with_compatible_surface(self, surface: Option<&'surface Surface<'window>>) -> Self;
    fn with_power_preference(self, power_preference: PowerPreference) -> Self;
    fn with_force_fallback_adapter(self, force_fallback_adapter: bool) -> Self;
}

impl<'surface, 'window> RequestAdapterOptionsExt<'surface, 'window> for RequestAdapterOptions<'surface, 'window> {
    fn with_compatible_surface(mut self, surface: Option<&'surface Surface<'window>>) -> Self {
        self.compatible_surface = surface;
        self
    }

    fn with_power_preference(mut self, power_preference: PowerPreference) -> Self {
        self.power_preference = power_preference;
        self
    }

    fn with_force_fallback_adapter(mut self, force_fallback_adapter: bool) -> Self {
        self.force_fallback_adapter = force_fallback_adapter;
        self
    }
}

impl<'surface, 'window> FromExt<&'surface Surface<'window>> for RequestAdapterOptions<'surface, 'window> {
    fn from_ext(surface: &'surface Surface<'window>) -> Self {
        wgpu::RequestAdapterOptions {
            force_fallback_adapter: Default::default(),
            power_preference: Default::default(),
            compatible_surface: Some(surface)
        }
    }
}
