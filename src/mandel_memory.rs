use tracing::info;

struct ComplexMandel {
    pub real: f64,
    pub imaginary: f64,
}

impl ComplexMandel {
    #[inline(always)]
    pub fn new(real: f64, imaginary: f64) -> ComplexMandel {
        ComplexMandel { real, imaginary }
    }

    #[inline(always)]
    pub fn multiply(&self, factor: &ComplexMandel) -> ComplexMandel {
        ComplexMandel::new(self.real * factor.real - self.imaginary * factor.imaginary, self.real * factor.imaginary + self.imaginary * factor.real)
    }

    #[inline(always)]
    pub fn add(&self, addend: &ComplexMandel) -> ComplexMandel {
        ComplexMandel::new(self.real + addend.real, self.imaginary + addend.imaginary)
    }
}

//const WIDTH: i32 = 80*100;
//const HEIGHT: i32 = 40*100;
const X_MIN: f64 = -2.0;
const X_MAX: f64 = 1.0;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MandelSet {
    pub result: String,
}

pub fn mandel(max_iter: u32, height: u32, width: u32) -> MandelSet {
    let mut result = String::with_capacity(height as usize * (width as usize + 1));

    let x_step: f64 = (X_MAX - X_MIN) / width as f64;
    let y_step: f64 = (Y_MAX - Y_MIN) / height as f64;

    let mut cy = Y_MIN;
    for _ in 0..height {
        let mut cx = X_MIN;

        for _ in 0..width {
            let c = ComplexMandel::new(cx, cy);
            let n = mandelbrot_set(&c, max_iter);

            let pixel = match n {
                0..=10 => ' ',
                11..=20 => '.',
                21..=30 => '+',
                31..=40 => '=',
                41..=50 => '?',
                51..=60 => '#',
                61..=70 => ':',
                // more characters for different iterations
                _ => '*',
            };
            //print!("{}", pixel);
            result.push(pixel);
            cx += x_step;
        }
        //println!();
        result.push('\n');
        cy += y_step;
    }

    //println!("{}", output);

    //info!("Strlen mandel = {}", result.len());

    MandelSet { result }
}

fn mandelbrot_set(c: &ComplexMandel, max_iter: u32) -> u32 {
    let mut z = ComplexMandel::new(0.0, 0.0);
    let mut n = 0;

    while (z.real * z.real + z.imaginary * z.imaginary < 4.0 && n < max_iter) {
        z = z.multiply(&z).add(c);
        n += 1;
    }

    n
}
