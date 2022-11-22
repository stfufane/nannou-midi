use nannou::{prelude::*, rand, state::Mouse};
use rand::Rng;

const RADIUS: f32 = 80.0;

pub struct Particle {
    pub position: Point2,
    pub previous: Point2,
    shift: Point2,
    pub size: f32,
    angle: f32,
    speed: f32,
    target_size: f32,
    orbit: f32,
    pub color: Srgb<u8>,
}

impl Particle {
    pub fn new(mouse: &Mouse) -> Self {
        // Generate a random RGB color.
        let mut rand_gen = rand::thread_rng();
        let red: u8 = rand_gen.gen();
        let green: u8 = rand_gen.gen();
        let blue: u8 = rand_gen.gen();

        // Create a default particle with some random elements.
        Particle {
            position: vec2(mouse.x, mouse.y),
            previous: vec2(mouse.x, mouse.y),
            shift: vec2(mouse.x, mouse.y),
            size: 2.0,
            angle: 0.0,
            speed: 0.01 + random_f32() * 0.04,
            target_size: 5.0,
            orbit: RADIUS * 0.5 + (RADIUS * 0.5 * random_f32()),
            color: Srgb::new(red, green, blue),
        }
    }

    pub fn update(&mut self, mouse: &Mouse, modifier: &Modifier) {
        self.previous = self.position;

        self.angle += self.speed * modifier.accelerator;

        self.shift.x += (mouse.x - self.shift.x) * self.speed * modifier.accelerator;
        self.shift.y += (mouse.y - self.shift.y) * self.speed * modifier.accelerator;

        self.position.x =
            self.shift.x + self.angle.cos() * self.orbit * modifier.scale;
        self.position.y =
            self.shift.y + self.angle.sin() * self.orbit * modifier.scale;

        self.size += (self.target_size - self.size) * 0.2;
        self.target_size = 1.0 + random_range(0.0, 5.0);
    }
}

pub struct Modifier 
{
    pub scale: f32,
    pub accelerator: f32,
}

impl Modifier {
    pub fn new() -> Self {
        Modifier {
            scale: 1.0,
            accelerator: 1.0,
        }
    }
}