use wgpu::{Adapter, Surface, SurfaceConfiguration};

pub trait SurfaceConfigurationExt: Sized {
    fn new(adapter: &Adapter, surface: &Surface, size: impl Into<(u32, u32)>) -> Self;
}

impl SurfaceConfigurationExt for SurfaceConfiguration {
    fn new(adapter: &Adapter, surface: &Surface, size: impl Into<(u32, u32)>) -> Self {
        let size = size.into();
        let surface_caps = surface.get_capabilities(adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        config.into()
    }
}
