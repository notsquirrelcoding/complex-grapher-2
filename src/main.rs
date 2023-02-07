mod grapher;

use std::f64::consts::{E, PI};

use grapher::Grapher;
use num_complex::{Complex, ComplexFloat};

fn main() -> anyhow::Result<()> {
    // let mut grapher = Grapher::new(100, 100, 131072.0, (PI / 2.0) * 131072.0, 0.0, false,
    //     |z| {
    //     z.ln()
    // });

    // grapher.run()

    let z = Complex::new(-10.0, 0.0);
    let r = 0.000000000001;
    let theta = PI / 2.0;

    println!(
        "{}",
        (f(z + r * E.powc(Complex::new(0.0, theta)))
            - f(z + r * E.powc(Complex::new(0.0, -theta))))
            / (r * 2.0)
    );

    Ok(())
}

fn f(z: Complex<f64>) -> Complex<f64> {
    z.ln()
}
