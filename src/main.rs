mod vec3;

use log::error;
use pixels::{Pixels, SurfaceTexture};
use vec3::{Vec3, Color};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
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
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // render the current frame
            draw_pixel(pixels.get_frame());
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

// Place this in some ray tracer struct/impl
fn draw_pixel(frame: &mut [u8])
{
    for(i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as i16; //i
        let y = (i / HEIGHT as usize) as i16; //j

        let pixel_color: Color = Color::new(
            x as f32 / (WIDTH - 1) as f32, 
            y as f32 / (HEIGHT - 1) as f32,
            0.25f32);

        pixel.copy_from_slice(&color_to_rgb_slice(&pixel_color));
    }
}

fn color_to_rgb_slice(color: &Color) -> [u8; 4] {
    [(255.99f32 * color.get_x_coord()) as u8, (255.99f32 * color.get_y_coord()) as u8, (255.99f32 * color.get_z_coord()) as u8, 0xff]
}
