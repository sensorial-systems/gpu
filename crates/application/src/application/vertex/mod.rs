use gpu::vertex_format;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub const ATTRIBUTES: &[wgpu::VertexAttribute] = &[
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: vertex_format![[f32; 3]],
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            shader_location: 1,
            format: vertex_format![[f32; 2]],
        }
    ];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [-1.0,  1.0, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 1.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 1.0,  1.0, 0.0], tex_coords: [1.0, 1.0] },
];

pub const INDICES: &[u16] = &[
    0, 2, 1,
    1, 2, 3
];