mod vec3;
mod ray;
mod hit_record;
mod sphere;
mod hittable_list;
mod rt_weekend;

use std::rc::Rc;

extern crate image;

use hit_record::HitRecord;
use hit_record::Hittable;
use image::RgbImage;
use rt_weekend::INFINITY;
use sphere::Sphere;
use vec3::{Point3D, Vec3, Color};
use ray::Ray;
use hittable_list::HittableList;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord::default();
    if world.hit(ray, 0.0, INFINITY, &mut record) {
        return &(&record.normal + &Color::new(1.0, 1.0, 1.0)) * 0.5;
    }     
    
    let unit_direction = ray.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    &(&Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Color::new(0.5, 0.7, 1.0) * t) 
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width:u32 = 1280;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(&Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Point3D::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height:f32 = 2.0;
    let viewport_width:f32 = aspect_ratio * viewport_height;
    let focal_length:f32 = 1.0; 

    let origin = Point3D::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = -Vec3::new(0.0, viewport_height, 0.0);
    let half_horizontal = &horizontal / 2.0;
    let half_vertical = &vertical / 2.0;

    let lower_left_corner = &(&(&origin - &half_horizontal) - &half_vertical) - &Vec3::new(0.0, 0.0, focal_length);

    let mut image_buffer: RgbImage = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height) {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f32) / (image_width as f32);
            let v = (j as f32) / (image_height as f32);

            let ray = Ray::new(
                &origin, 
                &(&(&lower_left_corner + &(&(&horizontal * u) + &(&vertical * v))) - &origin),
            );

            let pixel_color: Color = ray_color(&ray, &&world);

            let pixel = image_buffer.get_pixel_mut(i, j);
            *pixel = image::Rgb(color_to_rgb_slice(&pixel_color));
        }
    }

    image_buffer.save_with_format("./raytracing.png", image::ImageFormat::Png).unwrap();
}

fn color_to_rgb_slice(color: &Color) -> [u8; 3] {
    [
        (255.999 * color.x) as u8, 
        (255.999 * color.y) as u8, 
        (255.999 * color.z) as u8
    ]
}
