extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::ray::Ray;
use ray_tracer_challenge::sphere::Sphere;
use ray_tracer_challenge::tuple::Tuple;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let canvas_size: usize = 500;
    let wall_size = 10.0;
    let half_wall_size = wall_size / 2.0;
    let pixel_size = wall_size / (canvas_size as f32);

    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let red = Tuple::color(1.0, 0.0, 0.0);

    let sphere = Sphere::new();
    let ray_origin = Tuple::point(0.0, 0.0, -wall_size);

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let wall_point = Tuple::point(
                -half_wall_size + (pixel_size * (x as f32)),
                half_wall_size - (pixel_size * (y as f32)),
                wall_size,
            );
            let ray_direction = (wall_point - ray_origin).normalize();
            let ray = Ray::new(ray_origin, ray_direction);
            let xs = sphere.intersect(ray);
            if !xs.is_empty() {
                canvas.write_pixel(x, y, red);
            }
        }
    }

    let path = Path::new("sphere.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}", display, why.description())
        }
        Ok(file) => file,
    };

    match file.write_all(canvas.to_ppm().as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why.description())
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
