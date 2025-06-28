pub trait VertexFormatMap {
    const FORMAT: wgpu::VertexFormat;
}

#[macro_export]
macro_rules! vertex_format {
    ($type:ty) => {
        <$type as $crate::VertexFormatMap>::FORMAT
    };
}

#[macro_export]
macro_rules! vertex_format_map {
    ($type:ty, $format:expr) => {
        impl $crate::VertexFormatMap for $type {
            const FORMAT: wgpu::VertexFormat = $format;
        }
    };
}

vertex_format_map!(f32, wgpu::VertexFormat::Float32);
vertex_format_map!([f32; 2], wgpu::VertexFormat::Float32x2);
vertex_format_map!([f32; 3], wgpu::VertexFormat::Float32x3);
vertex_format_map!([f32; 4], wgpu::VertexFormat::Float32x4);

// TODO: f16 is unstable https://doc.rust-lang.org/std/primitive.f16.html
// vertex_format_map!(f16, wgpu::VertexFormat::Float16);
// vertex_format_map!([f16; 2], wgpu::VertexFormat::Float16x2);
// vertex_format_map!([f16; 4], wgpu::VertexFormat::Float16x4);

vertex_format_map!(u32, wgpu::VertexFormat::Uint32);
vertex_format_map!([u32; 2], wgpu::VertexFormat::Uint32x2);
vertex_format_map!([u32; 3], wgpu::VertexFormat::Uint32x3);
vertex_format_map!([u32; 4], wgpu::VertexFormat::Uint32x4);

vertex_format_map!(i32, wgpu::VertexFormat::Sint32);
vertex_format_map!([i32; 2], wgpu::VertexFormat::Sint32x2);
vertex_format_map!([i32; 3], wgpu::VertexFormat::Sint32x3);
vertex_format_map!([i32; 4], wgpu::VertexFormat::Sint32x4);

vertex_format_map!(u16, wgpu::VertexFormat::Uint16);
vertex_format_map!([u16; 2], wgpu::VertexFormat::Uint16x2);
vertex_format_map!([u16; 4], wgpu::VertexFormat::Uint16x4);

vertex_format_map!(i16, wgpu::VertexFormat::Sint16);
vertex_format_map!([i16; 2], wgpu::VertexFormat::Sint16x2);
vertex_format_map!([i16; 4], wgpu::VertexFormat::Sint16x4);

vertex_format_map!(u8, wgpu::VertexFormat::Uint8);
vertex_format_map!([u8; 2], wgpu::VertexFormat::Uint8x2);
vertex_format_map!([u8; 4], wgpu::VertexFormat::Uint8x4);
