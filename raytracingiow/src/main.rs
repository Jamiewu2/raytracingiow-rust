extern crate minifb;
extern crate rand;

mod math;
use math::Ray;
use math::Vec3;

use rand::Rng;

mod render;
use render::*;

use minifb::Key;
use minifb::Window;
use minifb::WindowOptions;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

const WIDTH: usize = 600;
const HEIGHT: usize = 300;

fn create_world() -> Box<dyn Renderable> {
    let center = Vec3::new(0_f64, 0_f64, -1_f64);
    let radius = 0.5;
    let lambertian_material = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let lambertian_material2 = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    let metal_material = Metal::new(Vec3::new(0.8, 0.8, 0.8));
    let metal_material2 = Metal::new(Vec3::new(0.8, 0.6, 0.2));

    let sphere = Sphere::new(center, radius, Box::new(lambertian_material));
    let sphere2 = Sphere::new(Vec3::new(0_f64, -100.5, -1_f64), 100_f64, Box::new(lambertian_material2));
    let sphere3 = Sphere::new(Vec3::new(1_f64, 0.0, -1_f64), 0.5, Box::new(metal_material));
    let sphere4 = Sphere::new(Vec3::new(-1_f64, 0.0, -1_f64), 0.5, Box::new(metal_material2));

    //chapter
    let world = vec![sphere, sphere2, sphere3, sphere4];
    return Box::new(world);
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Press Esc to exit", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("failed to create window, {}", e);
        });

    //TODO how the fk do i move this out of here and specify all the lifetimes
    let world = create_world();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        } else if window.is_key_down(Key::Key1) {
            buffer = create_buffer(WIDTH, HEIGHT);
        } else if window.is_key_down(Key::Key3) {
            buffer = create_ray_buffer(WIDTH, HEIGHT, &*world, get_bg_color);
        } else if window.is_key_down(Key::Key4) {
            buffer = create_ray_buffer(WIDTH, HEIGHT, &*world, get_color_chapter_4);
        } else if window.is_key_down(Key::Key5) {
            buffer = create_ray_buffer(WIDTH, HEIGHT, &*world, get_color_chapter_5);
        } else if window.is_key_down(Key::Key6) {
            buffer = create_ray_buffer_antialias(WIDTH, HEIGHT, &*world, get_color_chapter_5, 10);
        } else if window.is_key_down(Key::Key7) {
            buffer = create_ray_buffer_antialias(WIDTH, HEIGHT, &*world, get_color_chapter_7, 200);
        }

        window
            .update_with_buffer_size(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    //draw output
    draw_picture(WIDTH, HEIGHT, "output/chapter1.ppm", create_buffer).unwrap();

    let ray_buffer_closure_3 = |w, h| create_ray_buffer(w, h, &*world, get_bg_color);
    draw_picture(WIDTH, HEIGHT, "output/chapter3.ppm", ray_buffer_closure_3).unwrap();

    let ray_buffer_closure_4 = |w, h| create_ray_buffer(w, h, &*world, get_color_chapter_4);
    draw_picture(WIDTH, HEIGHT, "output/chapter4.ppm", ray_buffer_closure_4).unwrap();

    let ray_buffer_closure_5 = |w, h| create_ray_buffer(w, h, &*world, get_color_chapter_5);
    draw_picture(WIDTH, HEIGHT, "output/chapter5.ppm", ray_buffer_closure_5).unwrap();

    let ray_buffer_closure_6 = |w, h| create_ray_buffer_antialias(w, h, &*world, get_color_chapter_5, 10);
    draw_picture(WIDTH, HEIGHT, "output/chapter6.ppm", ray_buffer_closure_6).unwrap();

    let ray_buffer_closure_7 = |w, h| create_ray_buffer_antialias(w, h, &*world, get_color_chapter_7, 200);
    draw_picture(WIDTH, HEIGHT, "output/chapter7.ppm", ray_buffer_closure_7).unwrap();
}

trait RGB {
    fn to_u32_rgb(&self) -> u32;
    fn gamma_2_correct(&self) -> Self;
}

impl RGB for Vec3 {
    fn to_u32_rgb(&self) -> u32 {
        let ir = (255.99 * self.r()) as u8;
        let ig = (255.99 * self.g()) as u8;
        let ib = (255.99 * self.b()) as u8;

        let (r, g, b) = (ir as u32, ig as u32, ib as u32);
        return (r << 16) | (g << 8) | b;
    }

    fn gamma_2_correct(&self) -> Vec3 {
        return Vec3::new(self.r().sqrt(), self.g().sqrt(), self.b().sqrt());
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
                0.2,
            );

            let rgb = rgb_vec.to_u32_rgb();
            buffer.push(rgb);
        }
    }

    return buffer;
}

//chapter 3
//ignores world parameter, creates own world objects
fn get_bg_color(ray: &Ray, _world: &dyn Renderable) -> Vec3 {
    let white: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let blue: Vec3 = Vec3::new(0.5, 0.7, 1.0);

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0); // 0 to 1.0
                                              //lerp
    return (1.0 - t) * white + t * blue;
}

fn create_ray_buffer(x_size: usize, y_size: usize, world: &dyn Renderable, ray_fn: fn(&Ray, &dyn Renderable) -> Vec3) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    //u,v coordinate system, x: [-2, 2], y[-1, 1]
    let bottom_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(bottom_left, horizontal, vertical, origin);

    for j in (0..y_size).rev() {
        for i in 0..x_size {
            let u = (i as f64) / (x_size as f64);
            let v = (j as f64) / (y_size as f64);
            let ray = camera.get_ray(u, v);
            let color = ray_fn(&ray, world);
            let rgb = color.to_u32_rgb();
            buffer.push(rgb);
        }
    }

    return buffer;
}

//chapter 6
pub fn create_ray_buffer_antialias(x_size: usize, y_size: usize, world: &dyn Renderable, color_fn: fn(&Ray, &dyn Renderable) -> Vec3, alias_num: u32) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();

    //u,v coordinate system, x: [-2, 2], y[-1, 1]
    let bottom_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(bottom_left, horizontal, vertical, origin);

    for j in (0..y_size).rev() {
        for i in 0..x_size {

            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..alias_num {

                let rand_u: f64 = rng.gen();
                let rand_v: f64 = rng.gen();

                let u = (i as f64 + rand_u) / (x_size as f64);
                let v = (j as f64 + rand_v) / (y_size as f64);
                let ray = camera.get_ray(u, v);

                let color_sample = color_fn(&ray, world);

                color += color_sample;
            }

            color /= alias_num as f64;
            let rgb = color.gamma_2_correct().to_u32_rgb();
            buffer.push(rgb);
        }
    }

    return buffer;
}

//chapter 4
fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    //t*t*dot(B, B) + 2*t*dot(B,A-C) + dot(A-C,A-C) - R*R = 0

    let ac = ray.origin() - *center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * ray.direction().dot(ac);
    let c = ac.dot(ac) - radius * radius;

    let discriminant = b.powi(2) - 4_f64 * a * c;

    return discriminant > 0_f64;
}

fn get_color_chapter_4(ray: &Ray, world: &dyn Renderable) -> Vec3 {
    let center = Vec3::new(0_f64, 0_f64, -1_f64);
    let red = Vec3::new(1_f64, 0_f64, 0_f64);

    if hit_sphere(&center, 0.5, ray) {
        return red;
    } else {
        return get_bg_color(ray, world);
    }
}

//chapter 5
fn get_color_chapter_5(ray: &Ray, world: &dyn Renderable) -> Vec3 {

    match { world.hit(ray, 0_f64, std::f64::MAX) } {
        Some(hit_record) => {
            //hack, map surface_normal from [-1,1] xyz into range [0,1] rgb for visualization
            let surface_normal = hit_record.normal;
            return 0.5 * Vec3::new(
                    surface_normal.x() + 1_f64,
                    surface_normal.y() + 1_f64,
                    surface_normal.z() + 1_f64,
                );
        }
        None => return get_bg_color(ray, world),
    }
}

//chapter 7
fn get_color_chapter_7(ray: &Ray, world: &dyn Renderable) -> Vec3 {
    return get_color_chapter_7_tail(ray, world, 0);
}

fn get_color_chapter_7_tail(ray: &Ray, world: &dyn Renderable, num_bounces: i32) -> Vec3 {
    let max_bounces = 50;

    //add a little to the minimum to fix floating point inaccuracies
    match { world.hit(ray, 0.001_f64, std::f64::MAX) } {
        Some(hit_record) => {
            let (attenuation, scattered) = hit_record.material.scatter(ray, &hit_record);

            //recurse
            if num_bounces < max_bounces {
                return attenuation * get_color_chapter_7_tail(&scattered, world, num_bounces + 1);
            } else {
                return Vec3::new(0.0,0.0, 0.0);
            }
        }
        None => return get_bg_color(ray, world),
    }
}

fn draw_picture(
    x_size: usize,
    y_size: usize,
    filename: &str,
    f: impl Fn(usize, usize) -> Vec<u32>,
) -> io::Result<()> {
    let output_file = File::create(filename)?;

    {
        let mut writer = BufWriter::new(output_file);

        let header = format!(
            "P3\n{x_size} {y_size}\n255\n",
            x_size = x_size,
            y_size = y_size
        );
        writer.write_all(header.as_bytes())?;

        let buffer = f(x_size, y_size);

        for val in buffer {
            let ir = val.get_r();
            let ig = val.get_g();
            let ib = val.get_b();

            writer.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    return Ok(());
}
