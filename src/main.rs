// This is all just to create the images we load into textures.
// There has gotta be a better way :(
use image::{ImageBuffer, Rgba, ImageOutputFormat};
use std::time::{SystemTime, Instant, Duration};
use rand::Rng;


use crow::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context, DrawConfig, Texture, WindowSurface,
};

mod star;

fn main() -> Result<(), crow::Error> {

    let mut loaded = false;
    let  (mut desired_width, mut desired_height): (u32, u32) = (1920, 1080);

    let event_loop = EventLoop::new();
    let mut ctx = Context::new(WindowBuilder::new(), &event_loop)?;
    // RESIZE WINDOW
    ctx.resize_window(desired_width, desired_height);
    // TODO I hate to say it, but I think this needs
    // to be encapsulated into whatever is being drawn...
    let test_draw_config = DrawConfig {
        scale: (3, 3),
        ..Default::default()
    };
    let mut stars: Vec<star::Star> = Vec::new();
        //TODO this is a mess
    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
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
                for star in stars.iter_mut()  {
                    star.update(&mut ctx);
                }
                let mut surface = ctx.surface();
                ctx.clear_color(&mut surface, (0., 0., 0., 1.0));
                for star in stars.iter_mut() {
                    star.draw(&mut ctx, &mut surface, &test_draw_config);
                }
                ctx.present(surface).unwrap();
            }
            _ => {},
        },
    )
}

