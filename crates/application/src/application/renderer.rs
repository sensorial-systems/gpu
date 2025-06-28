use std::sync::Arc;

use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, PipelineLayoutDescriptor, SamplerBindingType, SamplerDescriptor, ShaderStages, TextureSampleType, TextureViewDescriptor, TextureViewDimension};
use wgpu::{util::BufferInitDescriptor, Adapter, Buffer, BufferUsages, Color, ColorTargetState, CommandEncoder, Device, FragmentState, Instance, LoadOp, Operations, PipelineLayout, PrimitiveState, PrimitiveTopology, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, ShaderModule, ShaderSource, StoreOp, Surface, SurfaceConfiguration, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, VertexState};
use winit::window::Window;
use gpu::*;
use crate::application::vertex;
use vertex::Vertex;

pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    surface_configuration: SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    bind_group: BindGroup,
}

impl Renderer {
   
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = Instance::new(&Default::default());
        let surface = Surface::new(&instance, window.clone())?;
        let adapter = Adapter::new(&instance, &surface).await?;
        let (device, queue) = Device::new(&adapter, &Default::default()).await?;

        let surface_configuration = SurfaceConfiguration::new(&adapter, &surface, size);

        let is_surface_configured = false;

        let shader = ShaderModule::new(&device, ShaderSource::Wgsl(include_str!("shader.wgsl").into()));

        let image = ::image::load_from_memory(include_bytes!("texture.png"))?;
        let image = image.to_rgba8();
        let dimensions = image.dimensions();
        let texture = Texture::new(
            &device,
            &TextureDescriptor::new(
                TextureDimension::D2,
                TextureFormat::Rgba8UnormSrgb,
                dimensions,
                TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST
            ).with_label(Some("Texture")));
        queue.write_texture_from_image(
            &texture,
            &image
        );
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = Sampler::new(&device, &SamplerDescriptor::new());

        let bind_group_layout = BindGroupLayout::new(
            &device,
            &BindGroupLayoutDescriptor::new(
                &[
                    BindGroupLayoutEntry::new(
                        0,
                        ShaderStages::FRAGMENT,
                        BindingType::texture(TextureSampleType::float(true), TextureViewDimension::D2, false)
                    ),
                    BindGroupLayoutEntry::new(
                        1,
                        ShaderStages::FRAGMENT,
                        BindingType::sampler(SamplerBindingType::Filtering),
                    ),
                ]
            ).with_label(Some("texture_bind_group_layout")));

        let bind_group = BindGroup::new(
            &device, 
            &BindGroupDescriptor::new(
                &bind_group_layout,
                &[
                    BindGroupEntry::new(0, &texture_view),
                    BindGroupEntry::new(1, &sampler),
                ]
            ).with_label(Some("texture_bind_group"))
        );

        let pipeline_layout = PipelineLayout::new(&device, &PipelineLayoutDescriptor::new().with_bind_group_layouts(&[&bind_group_layout]));

        let render_pipeline = RenderPipeline::new(
            &device,
            &RenderPipelineDescriptor::new(
                VertexState::new(&shader, "vs_main").with_buffers(&[Vertex::layout()]),
                PrimitiveState::new(PrimitiveTopology::TriangleList)
            )
                .with_layout(&pipeline_layout)
                .with_fragment(Some(FragmentState::new(&shader, "fs_main")
                    .with_targets(&[Some(ColorTargetState::new(surface_configuration.format))])
                )),
        );
    
        let vertex_buffer = Buffer::new(&device, &BufferInitDescriptor::new(vertex::VERTICES, BufferUsages::VERTEX));
        let index_buffer = Buffer::new(&device, &BufferInitDescriptor::new(vertex::INDICES, BufferUsages::INDEX));

        Ok(Self { surface, device, queue, surface_configuration, is_surface_configured, window, render_pipeline, vertex_buffer, index_buffer, bind_group })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_configuration.width = width;
            self.surface_configuration.height = height;
            self.surface.configure(&self.device, &self.surface_configuration);
            self.is_surface_configured = true;
        }
    }
    
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }
        
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = CommandEncoder::new(&self.device, "Render Encoder");

        {
            let mut render_pass = RenderPass::new(
                &mut encoder,
                &RenderPassDescriptor::new()
                    .with_color_attachments(&[Some(RenderPassColorAttachment::new(&view)
                        .with_ops(Operations::new(LoadOp::Clear(Color::new(0.0, 0.0, 0.0, 1.0)), StoreOp::Store)))]
                    )
            );
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..vertex::INDICES.len() as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();    

        Ok(())
    }
}
