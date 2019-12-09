extern crate minifb;

mod math;
use math::Vec3;
use math::Ray;

use std::io;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use minifb::Window;
use minifb::WindowOptions;
use minifb::Key;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

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
        } else if window.is_key_down(Key::Key3) {
            buffer = create_ray_buffer(WIDTH, HEIGHT);
        }

        window.update_with_buffer_size(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    //draw output
    draw_picture(WIDTH, HEIGHT, "output/chapter1.ppm")
        .unwrap();
}

trait RGB {
    fn to_u32_rgb(&self) -> u32;
}

impl RGB for Vec3 {
    fn to_u32_rgb(&self) -> u32 {
        let ir = (255.99 * self.r()) as u8;
        let ig = (255.99 * self.g()) as u8;
        let ib = (255.99 * self.b()) as u8;

        let (r, g, b) = (ir as u32, ig as u32, ib as u32);
        return (r << 16) | (g << 8) | b;
    }
}

trait RGBu32 {
    fn get_r(&self) -> u8;
    fn get_g(&self) -> u8;
    fn get_b(&self) -> u8;
}

impl RGBu32 for u32 {
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
            let rgb_vec = Vec3::new(
            (i as f64) / (x_size as f64),
            (j as f64) / (y_size as f64),
            0.2
            );

            let rgb = rgb_vec.to_u32_rgb();
            buffer.push(rgb);
        }
    }

    return buffer;
}

//chapter 3


fn get_color(ray: &Ray) -> Vec3 {
    let white: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let blue: Vec3 = Vec3::new(0.5, 0.7, 1.0);

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0); // 0 to 1.0
    //lerp
    return (1.0 - t) * white + t * blue;
}

fn create_ray_buffer(x_size: usize, y_size: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    //u,v coordinate system, x: [-2, 2], y[-1, 1]
    let bottom_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0,0.0,0.0);

    for j in (0..y_size).rev() {
        for i in 0..x_size {
            let u = (i as f64) / (x_size as f64);
            let v = (j as f64) / (y_size as f64);
            let direction = bottom_left + u * horizontal + v * vertical;
            let ray = Ray::new(origin, direction);
            let color = get_color(&ray);
            let rgb = color.to_u32_rgb();
            buffer.push(rgb);
        }
    }

    return buffer;
}

//() type inside the Result generic is a zero sized tuple. It's basically used in a similar vein to void
fn draw_picture(x_size: usize, y_size: usize, filename: &str) -> io::Result<()> {
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