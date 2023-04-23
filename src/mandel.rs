use tracing::info;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MandelSet {
    pub result: String,
}

pub fn mandel(max_iter: u32) -> MandelSet {
    let width = 80;
    let height = 40;
    let max_iter = max_iter; // 256 or 1000000 ...
    let x_min = -2.0;
    let x_max = 1.0;
    let y_min = -1.5;
    let y_max = 1.5;
    let x_step = (x_max - x_min) / width as f64;
    let y_step = (y_max - y_min) / height as f64;

    let mut result = String::with_capacity(3500);

    for y in 0..height {
        for x in 0..width {
            let c = (x_min + x_step * x as f64, y_min + y_step * y as f64);
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
