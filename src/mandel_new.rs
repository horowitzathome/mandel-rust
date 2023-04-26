use tracing::info;

const WIDTH: i32 = 80;
const HEIGHT: i32 = 40;
const X_MIN: f64 = -2.0;
const X_MAX: f64 = 1.0;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;
const X_STEP: f64 = (X_MAX - X_MIN) / WIDTH as f64;
const Y_STEP: f64 = (Y_MAX - Y_MIN) / HEIGHT as f64;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MandelSet {
    pub result: String,
}

pub fn mandel(max_iter: u32) -> MandelSet {
    let mut result = String::with_capacity(HEIGHT as usize * (WIDTH as usize + 1));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = (X_MIN + X_STEP * x as f64, Y_MIN + Y_STEP * y as f64);
            let n = mandelbrot_set(c, max_iter);

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
        }
        //println!();
        result.push('\n');
    }

    //println!("{}", output);

    //info!("Strlen mandel = {}", result.len());

    MandelSet { result }
}

fn mandelbrot_set(c: (f64, f64), max_iter: u32) -> u32 {
    let mut z = (0.0, 0.0);
    let mut n = 0;
    while z.0 * z.0 + z.1 * z.1 < 4.0 && n < max_iter {
        z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
        n += 1;
    }
    n
}
