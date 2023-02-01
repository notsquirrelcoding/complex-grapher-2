use std::{f64::consts::PI, path::Path};

use colors_transform::Color;
use console::Term;
use image::{ImageBuffer, ImageResult, Rgb, RgbImage};
use num_complex::Complex;

type ComplexFunc = fn(Complex<f64>) -> Complex<f64>;

pub struct Grapher {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    zoom_factor: f64,
    x_shift: f64,
    y_shift: f64,
    axis_enabled: bool,
    f: ComplexFunc,
}

impl Default for Grapher {
    fn default() -> Self {
        let mut buf = RgbImage::new(100, 100);

        buf.fill(255);

        Self {
            buf,
            width: 100,
            height: 100,
            zoom_factor: 1.0,
            x_shift: 0.0,
            y_shift: 0.0,
            axis_enabled: false,
            f: |z| z,
        }
    }
}

impl Grapher {
    pub fn new(
        w: u32,
        h: u32,
        zoom_factor: f64,
        x_shift: f64,
        y_shift: f64,
        axis_enabled: bool,
        f: ComplexFunc,
    ) -> Self {
        let mut buf = RgbImage::new(w, h);

        buf.fill(0);

        Self {
            buf,
            width: w,
            height: h,
            zoom_factor,
            x_shift,
            y_shift,
            axis_enabled,
            f,
        }
    }

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


    pub fn put_pixel(&mut self, x: u32, y: u32, color: Rgb<u8>) {
        if (x, y) == (22, 22) {
            println!("{color:?}")
        }
        self.buf.put_pixel(x, y, color);
    }

    pub fn width_frac_2(&self) -> i32 {
        self.width as i32 / 2
    }

    pub fn height_frac_2(&self) -> i32 {
        self.height as i32 / 2
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

    /// Updates the plot
    pub fn update_plot(&mut self) -> anyhow::Result<()> {
        // Update very pixel and we add an offset of +-1
        for x in -(self.width_frac_2())..=((self.width_frac_2()) - 1) {
            for y in (1 - (self.height_frac_2()))..=(self.height_frac_2()) {

                let num = Complex::new(
                    (x as f64 + self.x_shift) / self.zoom_factor,
                    (y as f64 + self.y_shift) / self.zoom_factor,
                );

                // Assign a color to f(z)
                let color = color_num((self.f)(num));

                let num = num * self.zoom_factor;

                let point = self.map_point(
                    (num.re - self.x_shift).round() as i32,
                    (num.im - self.y_shift).round() as i32,
                );

                self.put_pixel(point.0, point.1, color);

            }
        }

        if self.axis_enabled {
            self.draw_axes(5);
        }

        self.buf.save_with_format(Path::new("test.png"), image::ImageFormat::Png)?;
        Ok(())
    }

    pub fn _run(&mut self) -> anyhow::Result<()> {
        let stdout = Term::buffered_stdout();

        loop {
            if let Ok(character) = stdout.read_char() {
                match character {
                    'z' => {
                        self.zoom_factor *= 100.0;
                        self.x_shift *= 100.0;
                        self.y_shift *= 100.0;
                    }
                    'x' => {
                        self.zoom_factor /= 100.0;
                        self.x_shift /= 100.0;
                        self.y_shift /= 100.0;
                    }
                    'w' => self.y_shift += 1000.0,
                    'a' => self.x_shift -= 1000.0,
                    's' => self.y_shift -= 1000.0,
                    'd' => self.x_shift += 1000.0,
                    'e' => self.axis_enabled = !self.axis_enabled,
                    'r' => {
                        self.zoom_factor = 1.0;
                        self.x_shift = 0.0;
                        self.y_shift = 0.0;
                    }
                    'k' => break,
                    _ => {}
                }
                self.update_plot()?;

                print!("{}[2J", 27 as char);

                let num = Complex::new(
                    self.x_shift / self.zoom_factor,
                    self.y_shift / self.zoom_factor,
                );

                println!(
                    "ZOOM: {}\tCENTER (z): {}\t f(z)={}\tAXIS ENABLED: {}",
                    self.zoom_factor,
                    num,
                    (self.f)(num),
                    self.axis_enabled
                );
            }
        }

        Ok(())
    }
}

/// A function which uses domain coloring to color a pixel which represents a complex number
fn color_num(num: Complex<f64>) -> Rgb<u8> {
    let h = (num.arg() + (2.0 * PI) / 3.0) * 50.0;
    let l = ((2.0 * num.norm().atan()) / PI) * 50.0;

    let hsl = colors_transform::Hsl::from(h as f32, 100.0, l as f32);

    let rgb = hsl.to_rgb().as_tuple();

    Rgb([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8])
}
