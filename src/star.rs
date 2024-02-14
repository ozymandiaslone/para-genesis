// This is all just to create the images we load into textures.
// There has gotta be a better way :(
use image::{ImageBuffer, Rgba, ImageOutputFormat};
use std::io::Cursor;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, Instant, Duration};
use rand::Rng;

use noise::{NoiseFn, Perlin};

use crow::{
    Context, DrawConfig, Texture, WindowSurface,
};

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

pub struct Star {
    xpos: f32,
    ypos: f32,
    xvel: f32,
    yvel: f32,
    frames: Vec<Texture>,
    frame_idx: usize,
    last_update: Instant,
}




impl Star {

    pub fn new(
        xpos: f32,
        ypos: f32,
        xvel: f32,
        yvel: f32,
        ctx: &mut Context 
    ) -> Star {
        let frames = gen_rand_star_textures(ctx);
        Star {
            xpos,
            ypos,
            xvel,
            yvel,
            frames,
            frame_idx: 0,
            last_update: Instant::now(),
        }
    }
    
    // update the properties of the star
    pub fn update(&mut self, ctx: &mut Context) {

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        // update the frame to display 7x per second
        if elapsed >= Duration::from_secs_f32(1. / 7.) {
            self.last_update = now;
            if self.frame_idx < self.frames.len() - 1  {
                self.frame_idx += 1;
            } else {
                self.frame_idx = 0;
            }
        }
        let (win_width, win_height): (i32, i32) = ctx.window().inner_size().into();
        let (tex_x, tex_y) = self.frames[self.frame_idx].dimensions();
        self.xpos = self.xpos + self.xvel;
        self.ypos = self.ypos + self.yvel;
        // see we need to multiply by 3 here because 
        // the scaling in the draw config is messing it up
        // ideally we need to get the draw config in scope here
        if (self.xpos + tex_x as f32 * 3.) > win_width as f32 
            || self.xpos < 0. { self.xvel = -(self.xvel) };
        if (self.ypos + tex_y as f32 * 3.) > win_height as f32
            || self.ypos < 0. { self.yvel = -(self.yvel) };

    }
    // takes in a surface and a texture, 
    // then we just draw the texture which corresponds
    // to the current frame index
    pub fn draw(
        &mut self, 
        ctx: &mut Context, 
        surface: &mut WindowSurface, 
        conf: &DrawConfig 
    ) {
        ctx.draw(
            surface, 
            &self.frames[self.frame_idx], 
            (self.xpos as i32, self.ypos as i32), 
            conf,
        );
    }
}


pub fn gen_rand_star_textures(ctx: &mut Context) -> Vec<Texture> {
    let n_frames = 10;
    let mut textures: Vec<Texture> = Vec::new();
    let star_temp = rand::thread_rng().gen_range(4000..11000);

    for _ in 0..n_frames {
        let _ = generate_star_image(star_temp as f32); 
        let texture = Texture::load(ctx, "./textures/temp.png").unwrap();
        textures.push(texture);
    }
    textures
}


pub fn generate_star_image(temp: f32) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
    draw_star_to_image(
        &mut img, 
        (WIDTH / 2) as i32, 
        (HEIGHT / 2) as i32, 
        WIDTH as i32, 
        HEIGHT as i32, 
        29.,
        temp,
    );
    let dyn_image = image::DynamicImage::ImageRgba8(img);
    let mut buffer = Cursor::new(Vec::new());
    dyn_image.write_to(&mut buffer, ImageOutputFormat::Png)?;
    let image_mem = buffer.into_inner();
    let mut temp_file = File::create("textures/temp.png")?;
    temp_file.write_all(&image_mem)?;
    Ok(())
}

fn draw_star_to_image(
    img: &mut ImageBuffer<Rgba<u8>, 
    Vec<u8>>, 
    x0: i32, 
    y0: i32, 
    w: i32, 
    h: i32, 
    r: f32,
    temp: f32,
) {
    // need a random perlin seed for each star frame
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = (
        since_epoch.as_secs() * 1_000_000_000 + since_epoch
            .subsec_nanos() as u64 
    ) as u32;
    let perlin = Perlin::new(seed);
    let base_color = temp_to_color(temp);
    let (red, green, blue) = base_color;
    let p = r + r * 0.3;
    let v = p - r;
    for x in 0..w {
        for y in 0..h {
            // Calculating this pixel's (x,y) distance from the center of the circle (x0,y0)
            let xterms = ((x - x0) * (x - x0)) as f32;
            let yterms = ((y - y0) * (y - y0)) as f32;
            let d = fast_root( xterms + yterms );
            //
            //if this pixel is inside the radius of the circle...
            if d <= r {
                let noise_val = perlin.get([x as f64 / 2., y as f64 / 2.]);


                let mod_red = (red as f64 * (noise_val + 1.4) / 2.) as u8;
                let mod_green = (green as f64 * (noise_val + 1.4) / 2.) as u8;
                let mod_blue = (blue as f64 * (noise_val + 2.3) / 2.) as u8;
                
                img.get_pixel_mut(x as u32, y as u32).0 = [
                    mod_red,
                    mod_green,
                    mod_blue,
                    255,
                ];

            // if this pixel is outside the radius of the circle, but inside the radius p
            // (p defined above. p~r )
            } else if d <= p {
                let q = d - r;
                let noise_val = perlin.get([x as f64 / 2., y as f64 / 2.]);
                let alpha = (175. * (1. - q / v)) as u8;
                let mod_red = (red as f64 * (noise_val + 1.4) / 2.) as u8;
                let mod_green = (green as f64 * (noise_val + 1.4) / 2.) as u8;
                let mod_blue = (blue as f64 * (noise_val + 2.3) / 2.) as u8;
                
                img.get_pixel_mut(x as u32, y as u32).0 = [
                    mod_red,
                    mod_green,
                    mod_blue,
                    alpha,
                ];
            }
        }
    }
}

// Some really messed up stuff
// but it works, i guess
// and i don't want to spend more time
// thinking about star colors
fn temp_to_color(temp: f32) -> (u8, u8, u8) {
    // convert temperature to celsius (for no reason)
    let temp_celsius = temp - 273.15;

    // define temperature ranges for RGB channels
    let red_range = (1500.0, 3500.0);
    let green_range = (4000.0, 7000.0);
    let blue_range = (7500.0, 12000.0);

    // calculate normalized values for each channel
    let red = (temp_celsius - red_range.0) / (red_range.1 - red_range.0);
    let green = (temp_celsius - green_range.0) / (green_range.1 - green_range.0);
    let blue = (temp_celsius - blue_range.0) / (blue_range.1 - blue_range.0);

    // clamp values to the range [0, 1]
    let red = red.max(0.0).min(1.0);
    let green = green.max(0.0).min(1.0);
    let blue = blue.max(0.0).min(1.0);

    // convert normalized values to integers in the range [0, 255]
    let red_int = (red * 255.0) as u8;
    let green_int = (green * 255.0) as u8;
    let blue_int = (blue * 255.0) as u8;

    (red_int, green_int, blue_int)
}

fn fast_inverse_sqrt(n: f32) -> f32 {
    let i = unsafe { std::mem::transmute::<f32, i32>(n) };
    let j = 0x5f3759df - (i >> 1);
    let y = unsafe { std::mem::transmute::<i32, f32>(j) };
    y * (1.5f32 - 0.5f32 * n * y * y)
}

fn fast_root(n :f32) -> f32 {
    1. / fast_inverse_sqrt(n)
}

pub fn load_stars(loaded: &mut bool, stars: &mut Vec<Star>, ctx: &mut Context) {
    let (win_width, win_height): (i32, i32) = ctx.window().inner_size().into();
    stars.push(Star::new(0., 0., 0.001, 0.001, ctx));
    stars.push(Star::new(win_width as f32 - 300., 0., -0.001, 0.001, ctx));
    stars.push(Star::new(0., win_height as f32 - 300., 0.001, -0.001, ctx));
    stars.push(Star::new(win_width as f32 - 300., win_height as f32 - 300., -0.001, -0.001, ctx));
    *loaded = true;

}
