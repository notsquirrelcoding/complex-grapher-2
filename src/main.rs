mod grapher;

use grapher::Grapher;

// const Z: Complex<f64> = Complex::new(3.0, 2.0);

fn main() -> anyhow::Result<()> {
    let mut grapher = Grapher::new(10000, 10000, 1000.0, 0.0, 0.0, false, |z| {
        1.0 / z.powc(z.tan())
    });

    grapher.update_plot()
}
