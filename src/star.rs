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
use rand::distributions::{Distribution, Uniform};
use super::physics::*;
use super::camera::*;


const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;

pub struct Star {
    xpos: f32,
    ypos: f32,
    xvel: f32,
    yvel: f32,
    mass: u64,
    radius: f32,
    force_vectors: Vec<ForceVector>,
    frames: Vec<Texture>,
    frame_idx: usize,
    last_update: Instant,
}

impl PhysObj for Star {
    fn xpos(&self) -> f32 { self.xpos }
    fn ypos(&self) -> f32 { self.ypos }
    fn xvel(&self) -> f32 { self.xvel }
    fn yvel(&self) -> f32 { self.yvel }
    fn mass(&self) -> u64 { self.mass }

    fn add_vector(&mut self, force_vec: ForceVector) {
        self.force_vectors.push(force_vec);
    }
    fn force_vectors(&self) -> Vec<ForceVector> {
        self.force_vectors.clone()
    }

    //fn set_xvel(&mut self, xvel: f32) { self.xvel = xvel }
    //fn set_yvel(&mut self, yvel: f32) { self.yvel = yvel }
}

impl Star {

    pub fn new(
        xpos: f32,
        ypos: f32,
        xvel: f32,
        yvel: f32,
        mass: u64,
        radius: f32,
        ctx: &mut Context 
    ) -> Star {
        let frames = gen_rand_star_textures(radius, ctx);
        Star {
            xpos,
            ypos,
            xvel,
            yvel,
            frames,
            mass,
            radius,
            force_vectors: Vec::new(),
            frame_idx: 0,
            last_update: Instant::now(),
        }
    }
    
    pub fn update_physics(&mut self, ctx: &mut Context) {
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

        let mut final_vector: ForceVector = (0., 0.);
        //assume this self as a force_vectors full of vectors
        for i in 0..self.force_vectors.len() {
            final_vector.0 += self.force_vectors[i].0;
            final_vector.1 += self.force_vectors[i].1;
        }
        self.force_vectors = Vec::new();

        let ax = final_vector.0 / self.mass as f32;
        let ay = final_vector.1 / self.mass as f32;
        
        // v = at
        self.xvel += ax * elapsed.as_secs_f32(); 
        self.yvel += ay * elapsed.as_secs_f32();

        self.xpos += self.xvel * elapsed.as_secs_f32();
        self.ypos += self.yvel * elapsed.as_secs_f32();

    }

    // takes in a surface and a texture, 
    // then we just draw the texture which corresponds
    // to the current frame index
    pub fn draw(
        &mut self, 
        ctx: &mut Context, 
        surface: &mut WindowSurface, 
        camera: &mut Camera,
    ) {
        let (tex_x, tex_y) = self.frames[self.frame_idx].dimensions();

        let (scl_x, scl_y) = (1, 1);
        
        // crow does not allow downscaling of images 
        // cus it is 'pixel perfect' or something
        let draw_config = DrawConfig {
            //scale: (draw_width as u32, draw_height as u32),
            scale: (1, 1),
            ..Default::default()
        };
        ctx.draw(
            surface, 
            &self.frames[self.frame_idx], 
            (self.xpos as i32 - (tex_x as i32 * scl_x as i32 / 2), self.ypos as i32 - (tex_y as i32 * scl_y as i32 / 2)), 
            //(self.xpos as i32, self.ypos as i32),
            &draw_config,
        );
    }
}


pub fn gen_rand_star_textures(radius: f32, ctx: &mut Context) -> Vec<Texture> {
    let n_frames = 10;
    let mut textures: Vec<Texture> = Vec::new();
    let star_temp = rand::thread_rng().gen_range(4000..11000);

    for _ in 0..n_frames {
        let _ = generate_star_image(radius, star_temp as f32); 
        let texture = Texture::load(ctx, "./textures/temp.png").unwrap();
        textures.push(texture);
    }
    textures
}

// I wish there was a way to directly create crow textures
// but i am not knowledgeable enough to do so
// it seems crazy to me, to generate and literally write
// image files to disk, so that we can load those into textures
// and then just overwrite the image files when we are done
pub fn generate_star_image(radius: f32, temp: f32) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
    draw_star_to_image(
        &mut img, 
        (WIDTH / 2) as i32, 
        (HEIGHT / 2) as i32, 
        WIDTH as i32, 
        HEIGHT as i32, 
        radius,
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

//TODO this collison checking could be
//implemented generically i bet
pub fn check_collisions(stars: &mut Vec<Star>) {
    for i in 0..stars.len() {
        for j in i+1..stars.len() {
            let dx = stars[j].xpos - stars[i].xpos;
            let dy = stars[j].ypos - stars[i].ypos;

            let distance = (dx*dx + dy*dy).sqrt();

            let nx = dx / distance;
            let ny = dy / distance;

            if distance < (stars[i].radius + stars[j].radius) {
                // Calculate the overlap between the two circles (how much one circle
                // has penetrated into the other)
                let overlap = 0.5 * (distance - stars[i].radius - stars[j].radius);

                // Displace the current circle along the normal by half of the overlap
                stars[i].xpos -= overlap * nx / distance;
                stars[i].ypos -= overlap * ny / distance;

                // Displace the other circle along the normal by half of the overlap
                stars[j].xpos += overlap * nx / distance;
                stars[j].ypos += overlap * ny / distance;
            }

            let dx = stars[j].xpos - stars[i].xpos;
            let dy = stars[j].ypos - stars[i].ypos;

            let distance = (dx*dx + dy*dy).sqrt();


            if distance <= (stars[i].radius + stars[j].radius) {

                // Calculate the relative velocity
                let rvx = stars[j].xvel - stars[i].xvel;
                let rvy = stars[j].yvel - stars[i].yvel;

                // Calculate the relative velocity in terms of the normal direction
                let norm_vec = rvx * nx + rvy * ny;

                // Do not resolve if velocities are separating
                if norm_vec > 0. {
                    continue;
                }

                // Calculate the impulse scalar
                let e = 0.97;  // Coefficient of restitution
                let impulse = -(1. + e) * norm_vec / ((1. / stars[i].mass as f32) +  (1. / stars[j].mass as f32));

                // Apply impulse
                let impulse_x = impulse * nx;
                let impulse_y = impulse * ny;
                stars[i].xvel -= 1. / stars[i].mass as f32 * impulse_x;
                stars[i].yvel -= 1. / stars[i].mass as f32 * impulse_y;
                stars[j].xvel += 1. / stars[j].mass as f32 * impulse_x;
                stars[j].yvel += 1. / stars[j].mass as f32 * impulse_y;
            }
        }
    }
}

fn initialize_rand_star(win_width: i32, win_height: i32, ctx: &mut Context) -> Star {
    let vel_distribution = Uniform::new(0.0f32, 2.0f32);
    let mut rng = rand::thread_rng();
    let mass = rng.gen_range(100..100000000);
    let r = r_from_mass(mass as f32, (100., 100000000.), (1., 22.)) / 2.;
    Star::new(
        rng.gen_range(0..win_width) as f32,
        rng.gen_range(0..win_height) as f32,
        vel_distribution.sample(&mut rng) - 1.,
        vel_distribution.sample(&mut rng) - 1.,
        mass,
        r,
        ctx

    )
}

fn initialize_particle(win_width: i32, win_height: i32, ctx: &mut Context) -> Star {
    let mut rng = rand::thread_rng();
    Star::new(
        rng.gen_range(0..win_width) as f32,
        rng.gen_range(0..win_height) as f32,
        0.,
        0.,
        1000,
        1.,
        ctx
    )
}

pub fn load_stars(loaded: &mut bool, stars: &mut Vec<Star>, ctx: &mut Context) {
    let (win_width, win_height): (i32, i32) = ctx.window().inner_size().into();
    let desired_stars = 50;
    stars.push(Star::new((win_width as f32 / 2.) - 400., win_height as f32 / 2., 0., 2., 9999999999999, 55., ctx));
    stars.push(Star::new((win_width as f32 / 2.) + 400., win_height as f32 / 2., 0., -2., 9999999999999, 55., ctx));

    for _ in 0..desired_stars {
        stars.push(initialize_rand_star(win_width, win_height, ctx));
        //stars.push(initialize_particle(win_width, win_height, ctx));
    }
    *loaded = true;

}

fn r_from_mass(mass: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    let (from_min, from_max) = from_range;
    let (to_min, to_max) = to_range;

    (mass - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}
