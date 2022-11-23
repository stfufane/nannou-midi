use nannou::{prelude::*, rand};
use rand::Rng;

const RADIUS: f32 = 80.0;

pub struct Particle {
    pub position: Point2,
    pub previous: Point2,
    center: Point2,
    pub size: f32,
    angle: f32,
    speed: f32,
    target_size: f32,
    orbit: f32,
    pub color: Srgb<u8>,
}

impl Particle {
    pub fn new(bounds: &Rect<f32>) -> Self {
        // Generate a random RGB color.
        let mut rand_gen = rand::thread_rng();
        let red: u8 = rand_gen.gen();
        let green: u8 = rand_gen.gen();
        let blue: u8 = rand_gen.gen();

        let half_w = bounds.w() / 2.;
        let half_h = bounds.h() / 2.;
        let x = rand_gen.gen_range(-half_w..half_w);
        let y = rand_gen.gen_range(-half_h..half_h);

        // Create a default particle with some random elements.
        Particle {
            position: vec2(0., 0.),
            previous: vec2(0., 0.),
            center: vec2(x, y),
            size: 2.0,
            angle: 0.0,
            speed: 0.01 + random_f32() * 0.04,
            target_size: 5.0,
            orbit: RADIUS * 0.5 + (RADIUS * 0.5 * random_f32()),
            color: Srgb::new(red, green, blue),
        }
    }

    pub fn update(&mut self, modifier: &Modifier) {
        self.previous = self.position;

        self.angle += self.speed * modifier.accelerator;

        self.position.x = (self.center.x + modifier.center_shift.x) + self.angle.cos() * self.orbit * modifier.scale;
        self.position.y = (self.center.y + modifier.center_shift.y) + self.angle.sin() * self.orbit * modifier.scale;

        self.size += (self.target_size - self.size) * 0.2;
        self.target_size = 1.0 + random_range(0.0, 5.0);
    }

    pub fn draw(&self, draw: &Draw) {
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

pub struct Modifier 
{
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