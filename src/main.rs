// most of these imports aren't needed anymore but ill keep them for now
use image::{ImageBuffer, Rgba, ImageOutputFormat};
use std::time::{SystemTime, Instant, Duration};
use rand::Rng;
use crow::{
    glutin::{
        event::{Event, WindowEvent, VirtualKeyCode, MouseScrollDelta},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context, DrawConfig, Texture, WindowSurface,
};

mod star;
mod physics;
mod camera;

use physics::*;
use star::*;
use camera::*;


fn main() -> Result<(), crow::Error> {
    
    let mut camera = Camera::new();
    let mut loaded: bool = false;
    let  (mut desired_width, mut desired_height): (u32, u32) = (1920, 1080);

    let event_loop = EventLoop::new();
    let mut ctx = Context::new(WindowBuilder::new(), &event_loop)?;
    // RESIZE WINDOW
    ctx.resize_window(desired_width, desired_height);
    // TODO I hate to say it, but I think this needs
    // to be encapsulated into whatever is being drawn...
    // ill do it later. Not super pressing rn. 
    // may become pressing if I ever want to draw lots of things
    // at once, or at different scales
    //
    // now that i say it, ill prolly end up tackling this once
    // the concept of a camera exists here
    let test_draw_config = DrawConfig {
        scale: (1, 1),
        ..Default::default()
    };
    let mut stars: Vec<star::Star> = Vec::new();
    //TODO this is a mess
    // should prolly figure out what do do abt it
    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(virtual_keycode) = input.virtual_keycode {
                    match virtual_keycode {
                        VirtualKeyCode::A => {
                            println!("The 'A' key is pressed.");
                        },
                        // Add more match arms here for other keys
                        _ => {},
                    }
                }
            },
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                    let zoom = match delta {
                        MouseScrollDelta::LineDelta(_, y) => 1. + y * 0.1,
                        _ => 1.,
                    };
                    camera.setzoom(zoom);
                }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                if !loaded {
                    if ctx.window_width() == desired_width {
                        star::load_stars(&mut loaded, &mut stars, &mut ctx);
                    }
                }

                check_collisions(&mut stars);
                update_gravity_physics(&mut stars);

                for i in 0..stars.len()  {
                    stars[i].update_physics(&mut ctx);
                }

                let mut surface = ctx.surface();
                ctx.clear_color(&mut surface, (0., 0., 0., 1.0));
                for star in stars.iter_mut() {
                    star.draw(&mut ctx, &mut surface, &mut camera);
                }
                ctx.present(surface).unwrap();
            }
            _ => {},
        },
    )
}

