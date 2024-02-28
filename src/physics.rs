
pub type ForceVector = (f32, f32);

pub trait PhysObj {
    fn xpos(&self) -> f32;
    fn ypos(&self) -> f32;
    fn xvel(&self) -> f32;
    fn yvel(&self) -> f32;
    fn mass(&self) -> u64;
    fn force_vectors(&self) -> Vec<ForceVector>;
    fn add_vector(&mut self, force_vec: ForceVector);
}

pub fn calculate_gravity<T: PhysObj>(body1: &T, body2: &T) -> ForceVector {
    //TODO i wonder if, in this fn somewhere, we can
    //actually check for collisions too.
    let (x0, y0) = (body1.xpos(), body1.ypos());
    let (x1, y1) = (body2.xpos(), body2.ypos());
    let (m0, m1) = (body1.mass() as f32, body2.mass() as f32);
    
    let dx = x1 - x0;
    let dy = y1 - y0;
    let theta = f32::atan2(dy, dx);
    let r = (dx*dx + dy*dy).sqrt();
    let g = 0.000000001;
    let f = g * (m0 * m1) / (r * r);


    // these are the x,y components of the
    // force of gravity that body1 feels from 
    // body2 ( i hope )
    
    let fx = f * f32::cos(theta);
    let fy = f * f32::sin(theta);

    (fx, fy)
    // therefore... i think that the body2
    // components are just the negative 
    // components of body1 ?
    // ( im picturing just the opposite vec-
    // tor in my head )

}

pub fn update_gravity_physics<T: PhysObj>(bodies: &mut [T]) {
    // Calculate all the forces
    for i in 0..bodies.len() {
        for j in i+1..bodies.len() {
            let (fx, fy) = calculate_gravity(&bodies[i], &bodies[j]);
            let theta = f32::atan2(fy, fx);
            bodies[i].add_vector((fx, fy));
            bodies[j].add_vector((-fx, -fy));
        }
    }
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


