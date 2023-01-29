use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage, ImageResult};

pub struct Grapher {
    pub buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

impl Default for Grapher {
    fn default() -> Self {

        let mut buf = RgbImage::new(100, 100);

        buf.fill(255);

        Self { buf, width: 100, height: 100 }
    }
}

impl Grapher {

    pub fn draw_axes(&mut self, tick_space: u32) {

        let black_pixel = Rgb([0, 0, 0]);

        for y in 0..self.height {

            if y % tick_space == 0 {
                self.buf.put_pixel((self.width / 2) + 1, y, black_pixel);
                self.buf.put_pixel((self.width / 2) - 1, y, black_pixel);
            }

            self.buf.put_pixel(self.width / 2, y, black_pixel);
        }

        for x in 0..self.width {

            if x % tick_space == 0 {
                self.buf.put_pixel(x, (self.width / 2) + 1, black_pixel);
                self.buf.put_pixel(x, (self.width / 2) - 1, black_pixel);
            }

            self.buf.put_pixel(x, self.height / 2, black_pixel);
        }
    }

    pub fn save(&self, path: &Path) -> ImageResult<()> {
        self.buf.save_with_format(path, image::ImageFormat::Png)?;
        Ok(())
    }


    /// Maps a point from the coordinate system where `(0, 0)` is the center to the system where `(0, 0)` is the top-left corner
    /// of the screen
    pub fn map_point(&self, x: i32, y: i32) -> (u32, u32) {
        // Shifts the x coordinate 50 pixels to the left and flips the y coordinate around and shifts it up by 50 pixels as well
        (
            (x + (self.width / 2) as i32) as u32,
            (-y + (self.height / 2) as i32) as u32,
        )
    }
}