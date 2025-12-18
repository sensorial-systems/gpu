# Agents.md - GPU Crate Guide

This document helps LLM agents understand how to work with the `gpu` crate in this workspace.

## Overview

The `gpu` crate is a wrapper around `wgpu` (WebGPU for Rust) designed to simplify graphics programming. It provides a prelude of commonly used types and extension traits to streamline initialization.

## Key Concepts

The crate re-exports `wgpu` types, often extending them with helper methods.

### 1. Initialization (`gpu::Device`, `gpu::Adapter`)

Instead of the standard `wgpu` boilerplate, use the `DeviceExt` trait methods (available on `Device` via `use gpu::*;`).

```rust
use gpu::*;

// Initialize Instance and Surface (standard wgpu/winit)
let instance = Instance::new(&Default::default());
let surface = Surface::new(&instance, window.clone())?;

// Create Adapter
let adapter = Adapter::new(&instance, &surface).await?;

// Create Device and Queue using the helper extension
let (device, queue) = Device::new(&adapter, &Default::default()).await?;
```

### 2. File Structure (`crates/gpu/src`)

The crate is organized by `wgpu` concepts. Each module generally follows the matching `wgpu` type name:

- `adapter/`, `device/`, `queue/`: Core context.
- `pipeline/`, `render_pipeline/`, `shader_module/`: Pipeline states.
- `buffer/`, `texture/`, `image/`: Resources.
- `binding/`: Bind groups and layouts.

## Common Tasks

### Adding a new Shader
1.  Add the `.wgsl` file to the crate.
2.  Load it using `ShaderModule::new`.
    ```rust
    let shader = ShaderModule::new(&device, ShaderSource::Wgsl(include_str!("shader.wgsl").into()));
    ```

### creating a Buffer
Use `Buffer::new` with `BufferInitDescriptor` for easy initialization:
```rust
use wgpu::util::BufferInitDescriptor;
let vertex_buffer = Buffer::new(
    &device, 
    &BufferInitDescriptor::new(vertex::VERTICES, BufferUsages::VERTEX)
);
```

### Drawing
Inside the render pass, you can use standard `wgpu` commands, but ensure you're using the types re-exported or extended by this crate.

```rust
render_pass.set_pipeline(&self.render_pipeline);
render_pass.set_bind_group(0, &self.bind_group, &[]);
render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
render_pass.draw_indexed(0..indices.len(), 0, 0..1);
```
