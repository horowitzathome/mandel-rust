use tracing::info;

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

    for y in 0..height {
        for x in 0..width {
            let c = (X_MIN + x_step * x as f64, Y_MIN + y_step * y as f64);
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

    info!("Strlen mandel = {}", result.len());

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
