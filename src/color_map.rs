use image::imageops::colorops::ColorMap;
use image::Luma;

/// A bi-level color map with parameterized threshold.
///
/// Based on the [`BiLevel`] colormap from the [`image`] crate.
/// The threshold parameter describes the level above which the color is considered light.
///
/// [`BiLevel`]: https://docs.rs/image/0.23.3/image/imageops/colorops/struct.BiLevel.html
/// [`image`]: https://docs.rs/image
#[derive(Clone, Copy)]
pub struct DynamicBiLevel {
    pub threshold: u8,
}

impl ColorMap for DynamicBiLevel {
    type Color = Luma<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Luma<u8>) -> usize {
        let luma = color.0;
        if luma[0] > self.threshold {
            1
        } else {
            0
        }
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Luma<u8>) {
        let new_color = 0xFF * self.index_of(color) as u8;
        let luma = &mut color.0;
        luma[0] = new_color;
    }
}
