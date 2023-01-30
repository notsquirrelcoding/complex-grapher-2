mod grapher;

use std::{f64::consts::PI, path::Path};

use colors_transform::Color;
use console::Term;
use grapher::Grapher;
use image::Rgb;
use num_complex::{Complex, ComplexFloat};

const Z: Complex<f64> = Complex::new(3.0, 2.0);

fn main() -> anyhow::Result<()> {
    let mut grapher = Grapher::default();

    let mut zoom_factor = 20.0;
    let mut x_shift = 0.0;
    let mut y_shift = 0.0;
    let mut axis_enabled = false;

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'z' => {
                    zoom_factor *= 2.0;
                    x_shift *= 2.0;
                    y_shift *= 2.0;
                }
                'x' => {
                    zoom_factor /= 2.0;
                    x_shift /= 2.0;
                    y_shift /= 2.0;
                }
                'w' => y_shift += 10.0,
                'a' => x_shift -= 10.0,
                's' => y_shift -= 10.0,
                'd' => x_shift += 10.0,
                'e' => axis_enabled = !axis_enabled,
                'r' => {
                    zoom_factor = 1.0;
                    x_shift = 0.0;
                    y_shift = 0.0;
                }
                _ => break,
            }
            update_plot(&mut grapher, zoom_factor, x_shift, y_shift, axis_enabled)?;
            print!("{}[2J", 27 as char);

            let num = Complex::new(x_shift / zoom_factor, y_shift / zoom_factor);

            println!(
                "ZOOM: {zoom_factor}\tCENTER (z): {}\t f(z)={}\tAXIS ENABLED: {}",
                num,
                f(num),
                axis_enabled
            );
        }
    }

    Ok(())
}

fn update_plot(
    grapher: &mut Grapher,
    zoom_factor: f64,
    x_shift: f64,
    y_shift: f64,
    draw_axes: bool,
) -> anyhow::Result<()> {
    for x in -(grapher.width_frac_2())..=((grapher.width_frac_2()) - 1) {
        for y in (1-(grapher.height_frac_2()))..=(grapher.height_frac_2()) {
            let num = Complex::new(
                (x as f64 + x_shift) / zoom_factor,
                (y as f64 + y_shift) / zoom_factor,
            );

            let color = color_num(f(num));

            let num = num * zoom_factor;

            let point = grapher.map_point((num.re - x_shift) as i32, (num.im - y_shift) as i32);

            grapher.put_pixel(point.0, point.1, color);
        }
    }

    if draw_axes {
        grapher.draw_axes(5);
    }

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

fn f(z: Complex<f64>) -> Complex<f64> {
    1.0 / z.ln()
}
