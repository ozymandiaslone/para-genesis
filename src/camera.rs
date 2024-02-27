

pub struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera{
            x: 0.,
            y: 0.,
            zoom: 1.,
        }
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn setx(&mut self, x: f32) {
        self.x = x;
    }
    pub fn sety(&mut self, y: f32) {
        self.y = y;
    }
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    pub fn setzoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}
