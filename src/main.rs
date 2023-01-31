mod grapher;

use grapher::Grapher;
use num_complex::ComplexFloat;

// const Z: Complex<f64> = Complex::new(3.0, 2.0);

fn main() -> anyhow::Result<()> {
    let mut grapher = Grapher::new(4000, 4000, 400.0, 0.0, 0.0, false, |z| {
        1.0 / z.powc(z.tan())
    });

    grapher.update_plot()
}
