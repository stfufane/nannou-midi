use nannou::{prelude::*, rand};
use rand::Rng;

const RADIUS: f32 = 50.0;

fn rand_color() -> Srgb<u8> {
    // Generate a random RGB color.
    let mut rand_gen = rand::thread_rng();
    let red: u8 = rand_gen.gen();
    let green: u8 = rand_gen.gen();
    let blue: u8 = rand_gen.gen();
    Srgb::new(red, green, blue)
}

pub struct RotatingParticle {
    pub held: bool,
    position: Point2,
    previous: Point2,
    center: Point2,
    size: f32,
    angle: f32,
    speed: f32,
    orbit: f32,
    color: Srgb<u8>,
    pub lifetime: i32,
}

impl RotatingParticle {
    pub fn new(center: Point2, velocity: u8) -> Self {
        // Create a default particle with some random elements.
        RotatingParticle {
            held: true,
            position: center,
            previous: center,
            center,
            size: 2.0,
            angle: 0.,
            speed: 0.01 + random_f32() * 0.04,
            orbit: RADIUS,
            color: rand_color(),
            lifetime: velocity as i32,
        }
    }

    pub fn update(&mut self, modifier: &Modifier) {
        if !self.held {
            self.lifetime -= 1;
        } // The particle will be removed when it reaches 0.
        self.previous = self.position;

        self.angle += self.speed * modifier.accelerator;

        self.position.x = (self.center.x + modifier.center_shift.x)
            + self.angle.cos() * self.orbit * modifier.scale;
        self.position.y = (self.center.y + modifier.center_shift.y)
            + self.angle.sin() * self.orbit * modifier.scale;
    }

    pub fn draw(&self, draw: &Draw) {
        // Do not draw the first time because position has not been calculated yet.
        if self.previous == self.center {
            return;
        }

        // Draw a line from the last position to the new one + a circle at the end.
        draw.line()
            .points(self.previous, self.position)
            .stroke_weight(self.size)
            .color(self.color);

        draw.ellipse()
            .x_y(self.position.x, self.position.y)
            .radius(self.size / 2.0)
            .color(self.color);
    }
}

pub struct RippleCircle {
    pub held: bool,
    center: Point2,
    size: f32,
    orbit: f32,
    color: Srgb<u8>,
    pub lifetime: i32,
}

impl RippleCircle {
    pub fn new(center: Point2, velocity: u8) -> Self {
        RippleCircle {
            held: true,
            center,
            size: 2.,
            orbit: 5.,
            color: rand_color(),
            lifetime: velocity as i32,
        }
    }

    pub fn update(&mut self, _modifier: &Modifier) {
        if !self.held {
            self.lifetime -= 1;
        }
        self.orbit *= 1.02;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.center.x, self.center.y)
            .radius(self.orbit)
            .no_fill()
            .stroke_weight(self.size)
            .stroke_color(self.color);
    }
}

pub struct Modifier {
    pub scale: f32,
    pub accelerator: f32,
    pub center_shift: Point2,
}

impl Modifier {
    pub fn new() -> Self {
        Modifier {
            scale: 1.0,
            accelerator: 1.0,
            center_shift: vec2(0.0, 0.0),
        }
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::new()
    }
}
