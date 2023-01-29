mod grapher;

use std::{f64::consts::PI, path::Path};

use colors_transform::Color;
use grapher::Grapher;
use image::Rgb;
use num_complex::{Complex, ComplexFloat};

const ZOOM_FACTOR: f64 = 0.1;
const X_SHIFT: f64 = 0.0;
const Y_SHIFT: f64 = 0.0;

fn main() -> anyhow::Result<()> {
    let mut grapher = Grapher::default();
    
    for x in -50..=49 {
        for y in -49..=50 {
            let num = Complex::new(
                (x as f64 + X_SHIFT) / ZOOM_FACTOR,
                (y as f64 + Y_SHIFT) / ZOOM_FACTOR,
            );
            let color = color_num(num.sin());

            let num = num * ZOOM_FACTOR;
            
            let point = grapher.map_point((num.re - X_SHIFT) as i32, (num.im - Y_SHIFT) as i32);
            
            grapher.buf.put_pixel(point.0, point.1, color);
        }
    }
    
    grapher.draw_axes(10);
    grapher.save(Path::new("test.png"))?;

    Ok(())
}

/// A function which uses domain coloring to color a pixel which represents a complex number
fn color_num(num: Complex<f64>) -> Rgb<u8> {
    let h = (num.arg() + (2.0 * PI) / 3.0) * 50.0;
    let l = ((2.0 * num.norm().atan()) / PI) * 50.0;

    let hsl = colors_transform::Hsl::from(h as f32, 100.0, l as f32);

    let rgb = hsl.to_rgb().as_tuple();

    Rgb([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8])
}
