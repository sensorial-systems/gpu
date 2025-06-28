use crate::prelude::*;

use wgpu::Color;

pub trait ColorExt {
    fn new(r: f64, g: f64, b: f64, a: f64) -> Self;
}

impl ColorExt for Color {
    fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }
}

impl FromExt<f64> for Color {
    fn from_ext(value: f64) -> Self {
        Color { r: value, g: value, b: value, a: 1.0 }
    }
}

impl FromExt<f32> for Color {
    fn from_ext(value: f32) -> Self {
        Color { r: value as f64, g: value as f64, b: value as f64, a: 1.0 }
    }
}

impl FromExt<(f32, f32, f32)> for Color {
    fn from_ext(value: (f32, f32, f32)) -> Self {
        Color { r: value.0 as f64, g: value.1 as f64, b: value.2 as f64, a: 1.0 }
    }
}


impl FromExt<(f64, f64, f64)> for Color {
    fn from_ext(value: (f64, f64, f64)) -> Self {
        Color { r: value.0, g: value.1, b: value.2, a: 1.0 }
    }
}

impl FromExt<(f32, f32, f32, f32)> for Color {
    fn from_ext(value: (f32, f32, f32, f32)) -> Self {
        Color { r: value.0 as f64, g: value.1 as f64, b: value.2 as f64, a: value.3 as f64 }
    }
}

impl FromExt<(f64, f64, f64, f64)> for Color {
    fn from_ext(value: (f64, f64, f64, f64)) -> Self {
        Color { r: value.0, g: value.1, b: value.2, a: value.3 }
    }
}