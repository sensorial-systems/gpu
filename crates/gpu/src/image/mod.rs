use std::ops::Deref;

use image::Rgba;

pub trait ImageLayout {
    fn data(&self) -> &[u8];
    fn bytes_per_pixel(&self) -> u32;
    fn dimensions(&self) -> (u32, u32);
}

// TODO: Generalize it for image::ImageBuffer<P, Container>
impl ImageLayout for image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn data(&self) -> &[u8] {
        self.deref()
    }

    fn bytes_per_pixel(&self) -> u32 {
        4
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width() as u32, self.height() as u32)
    }
}