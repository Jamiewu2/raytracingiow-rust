extern crate minifb;

use std::io;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use minifb::Window;
use minifb::WindowOptions;
use minifb::Key;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Press Esc to exit", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("failed to create window, {}", e);
        });

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        } else if window.is_key_down(Key::Key1) {
             buffer = create_buffer(WIDTH, HEIGHT);
        }

        window.update_with_buffer_size(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    //draw output
    draw_picture(200, 100, "output/chapter1.ppm")
        .unwrap();
}

trait RGB {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> Self;

    fn get_r(&self) -> u8;
    fn get_g(&self) -> u8;
    fn get_b(&self) -> u8;
}

impl RGB for u32 {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn get_r(&self) -> u8 {
        return ((self >> 16) & 0xFF as u32) as u8;
    }
    fn get_g(&self) -> u8 {
        return ((self >> 8) & 0xFF as u32) as u8;
    }
    fn get_b(&self) -> u8 {
        return (self & 0xFF as u32) as u8;
    }
}

//chapter 1
fn create_buffer(x_size: usize, y_size: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for j in (0..y_size).rev() {
        for i in 0..x_size {

            //cast into range from 0 to 1.0
            let r = (i as f64) / (x_size as f64);
            let g = (j as f64) / (y_size as f64);
            let b = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            let rgb = RGB::from_u8_rgb(ir, ig, ib);
            buffer.push(rgb);
        }
    }

    return buffer;
}

//() type inside the Result generic is a zero sized tuple. It's basically used in a similar vein to void
fn draw_picture(x_size: i32, y_size: i32, filename: &str) -> io::Result<()> {
    //? syntax: try, unwrap if success, otherwise pass the error up the call stack
    let output_file = File::create(filename)?;

    {
        let mut writer = BufWriter::new(output_file);

        let header = format!("P3\n{x_size} {y_size}\n255\n", x_size=x_size, y_size=y_size);
        writer.write_all(header.as_bytes())?;

        let buffer = create_buffer(x_size as usize, y_size as usize);

        for val in buffer {
            let ir = val.get_r();
            let ig = val.get_g();
            let ib = val.get_b();

            writer.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    return Ok(());
}