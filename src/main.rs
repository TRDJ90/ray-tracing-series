mod vec3;
mod ray;

use log::error;
use pixels::{Pixels, SurfaceTexture};
use vec3::Point3D;
use vec3::dot;
use vec3::{Vec3, Color};
use ray::Ray;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width:u32 = 640;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let viewport_height:f32 = 2.0;
    let viewport_width:f32 = aspect_ratio * viewport_height;
    let focal_length:f32 = 1.0; 

    let origin = Point3D::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0) * -1.0;
    let lower_left_corner: Vec3 = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(image_width as f64, image_height as f64);
        WindowBuilder::new()
            .with_title("Ray tracing")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // pixel we can manipulate
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // render the current frame
            draw_pixel(
                pixels.get_frame(), 
                image_width, 
                image_height,
                origin,
                horizontal,
                vertical,
                lower_left_corner,
            );
            
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close event
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // request a redraw
            window.request_redraw();
        }
    });
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3D::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(
            normal.get_x_coord() + 1.0,
            normal.get_y_coord() + 1.0,
            normal.get_z_coord() + 1.0,
        )
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    let t: f32 = 0.5 * (unit_direction.get_y_coord() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(sphere_center: &Point3D, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3 = ray.origin() - *sphere_center;
    let a = ray.direction().length_squared();
    let half_b = dot(&oc, &ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = (half_b * half_b) - a * c;

    if discriminant < 0.0f32 {
       return -1.0; 
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
    
}

// Place this in some ray tracer struct/impl
fn draw_pixel(
    frame: &mut [u8], 
    image_width: u32, 
    image_height: u32,
    origin:Point3D,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
) {
    for(index, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let i = index % image_width as usize; //i
        let j = index / image_height as usize; //j

        let u = i as f32 / (image_width as f32);
        let v = j as f32 / (image_width as f32) ; 
        
        let ray = Ray::new(
            origin, 
            lower_left_corner + (u * horizontal) + (v * vertical) - origin,
        );

        let pixel_color: Color = ray_color(&ray);
        pixel.copy_from_slice(&color_to_rgb_slice(&pixel_color));
    }
}

fn color_to_rgb_slice(color: &Color) -> [u8; 4] {
    [
        (255.999 * color.get_x_coord()) as u8, 
        (255.999 * color.get_y_coord()) as u8, 
        (255.999 * color.get_z_coord()) as u8, 
        0xff
    ]
}
