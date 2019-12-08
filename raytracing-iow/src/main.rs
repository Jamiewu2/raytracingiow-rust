use std::io;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    draw_picture(200, 100, "output/chapter1.ppm")
        .unwrap();
}

//() type inside the Result generic is a zero sized tuple. It's basically used in a similar vein to void
fn draw_picture(x_size: i32, y_size: i32, filename: &str) -> io::Result<()> {
    //? syntax: try, unwrap if success, otherwise pass the error up the call stack
    let output_file = File::create(filename)?;

    {
        let mut writer = BufWriter::new(output_file);

        let header = format!("P3\n{x_size} {y_size}\n255\n", x_size=x_size, y_size=y_size);
        writer.write_all(header.as_bytes())?;

        for j in (0..y_size).rev() {
            for i in 0..x_size {

                //cast into range from 0 to 1.0
                let r = (i as f64) / (x_size as f64);
                let g = (j as f64) / (y_size as f64);
                let b = 0.2;

                let ir = (255.99 * r) as i32;
                let ig = (255.99 * g) as i32;
                let ib = (255.99 * b) as i32;

                writer.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
            }
        }
    }

    return Ok(());
}