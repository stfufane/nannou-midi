use nannou::{prelude::*, state::Mouse, rand};
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const NB_PARTICLES: usize = 25;

const RADIUS: f32 = 80.0;
const MAX_RADIUS_SCALE: f32 = 2.0;

struct Particle {
    position: Point2,
    previous: Point2,
    shift: Point2,
    size: f32,
    angle: f32,
    speed: f32,
    target_size: f32,
    orbit: f32,
    color: Srgb<u8>
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
            color: Srgb::new(red, green, blue) 
        }
    }
}

struct Model {
    particles: Vec<Particle>,
    radius_scale: f32, // Global orbit radius scale to apply to each particle.
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(SCREEN_WIDTH, SCREEN_HEIGHT).view(view).build().unwrap();
    let mut particles = Vec::with_capacity(NB_PARTICLES);

    for _i in 0..NB_PARTICLES {
        let particle = Particle::new(&app.mouse);
        particles.push(particle);
    }

    Model {  
        particles,
        radius_scale: 1.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // The radius goes up a little when we're clicking.
    let mouse_down = app.mouse.buttons.pressed().count() > 0;
    if mouse_down {
        model.radius_scale += (MAX_RADIUS_SCALE - model.radius_scale) * 0.02;
    } else {
        model.radius_scale -= (model.radius_scale - 1.0) * 0.02;
    }
    model.radius_scale = model.radius_scale.min(MAX_RADIUS_SCALE);

    // Compute the new positions for each particle, following the mouse position 
    // and orbiting around.
    model.particles.iter_mut().for_each(|particle| {
        particle.previous = particle.position;

        particle.angle += particle.speed;

        particle.shift.x += (app.mouse.x - particle.shift.x) * particle.speed;
        particle.shift.y += (app.mouse.y - particle.shift.y) * particle.speed;

        particle.position.x = particle.shift.x + particle.angle.cos() * particle.orbit * model.radius_scale;
        particle.position.y = particle.shift.y + particle.angle.sin() * particle.orbit * model.radius_scale;

        particle.size += (particle.target_size - particle.size) * 0.2;
        particle.target_size = 1.0 + random_range(0.0, 5.0);
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    // Draw a transparent black rectangle to make the particles fade.
    draw.rect().w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).color(srgba(0.0,0.0,0.0,0.05));

    // Draw a line from the last position to the new one + a circle at the end.
    model.particles.iter().for_each(|particle| {
        draw.line()
            .points(particle.previous, particle.position)
            .stroke_weight(particle.size)
            .color(particle.color);

        draw.ellipse()
            .x_y(particle.position.x, particle.position.y)
            .radius(particle.size / 2.0)
            .color(particle.color);
    });

    draw.to_frame(app, &frame).unwrap();
}