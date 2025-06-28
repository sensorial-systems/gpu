// TODO: Start creating categories for the codebase like "binding".

pub mod prelude;

mod surface;
mod adapter;
mod device;
mod queue;
mod shader_module;
mod pipeline;
mod color_target_state;
mod multisample_state;
mod primitive_state;
mod render_pipeline;
mod render_pass;
mod buffer;
mod vertex_format_map;
mod convert;
mod label;
mod color;
mod extent_3d;
mod texture;
mod image;
mod command_encoder;
mod binding;

pub use surface::*;
pub use adapter::*;
pub use device::*;
pub use queue::*;
pub use shader_module::*;
pub use pipeline::*;
pub use color_target_state::*;
pub use multisample_state::*;
pub use primitive_state::*;
pub use render_pipeline::*;
pub use render_pass::*;
pub use buffer::*;
pub use vertex_format_map::*;
pub use label::*;
pub use color::*;
pub use texture::*;
pub use image::*;
pub use command_encoder::*;
pub use binding::*;
