use crate::prelude::*;

use wgpu::Extent3d;

impl FromExt<(u32, u32, u32)> for Extent3d {
    fn from_ext((width, height, depth_or_array_layers): (u32, u32, u32)) -> Self {
        Extent3d { width, height, depth_or_array_layers }
    }
}

impl FromExt<(u32, u32)> for Extent3d {
    fn from_ext((width, height): (u32, u32)) -> Self {
        Extent3d { width, height, depth_or_array_layers: 1 }
    }
}

impl FromExt<u32> for Extent3d {
    fn from_ext(width: u32) -> Self {
        Extent3d { width, height: 1, depth_or_array_layers: 1 }
    }
}
