use midir::MidiInputConnection;
use nannou::prelude::*;
use particles::{Particle, Modifier};
use std::sync::mpsc::{channel, Receiver};
use wmidi::{ControlFunction, MidiMessage, U7};

pub mod midi;
pub mod particles;

fn main() {
    nannou::app(model).update(update).run();
}

const NB_PARTICLES: usize = 25;
const MAX_RADIUS_SCALE: f32 = 10.0;
const MAX_ACCELERATOR: f32 = 5.0;

struct Model {
    particles: Vec<Particle>,
    modifier: Modifier,
    _connection: Option<MidiInputConnection<()>>,
    receiver: Receiver<Vec<u8>>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .fullscreen()
        .view(view)
        .build()
        .unwrap();
    let mut particles = Vec::with_capacity(NB_PARTICLES);

    for _i in 0..NB_PARTICLES {
        let particle = Particle::new(&app.mouse);
        particles.push(particle);
    }

    let (tx, rx) = channel();

    Model {
        particles,
        modifier: Modifier::new(),
        _connection: midi::init(tx),
        receiver: rx,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for data in model.receiver.try_recv().iter() {
        match MidiMessage::try_from(data.as_slice()) {
            Err(_) => {
                print!("Invalid midi message");
            }
            Ok(midi_message) => {
                match midi_message {
                    wmidi::MidiMessage::ControlChange(_, control, value) => {
                        if control == ControlFunction::EFFECTS_3_DEPTH {
                            model.modifier.scale = 1.0 + (MAX_RADIUS_SCALE - 1.0)
                                * <u8 as From<wmidi::U7>>::from(value) as f32
                                / <u8 as From<wmidi::U7>>::from(U7::MAX) as f32;
                        } else if control == ControlFunction::SOUND_CONTROLLER_5 {
                            model.modifier.accelerator = 1.0 + (MAX_ACCELERATOR - 1.0)
                                * <u8 as From<wmidi::U7>>::from(value) as f32
                                / <u8 as From<wmidi::U7>>::from(U7::MAX) as f32;
                        }
                    }
                    wmidi::MidiMessage::NoteOn(_channel, _note, _velocity) => {
                        // println!("Channel {:?}, note {:?}, velocity {:?}", channel, note, velocity);
                    }
                    _ => (),
                }
            }
        };
    }

    // TODO: cool variations
    // let mouse_down = app.mouse.buttons.pressed().count() > 0;
    // model.radius_scale = (model.radius_scale.min(MAX_RADIUS_SCALE)).max(1.0);

    // Compute the new positions for each particle, following the mouse position
    // and orbiting around.
    model.particles.iter_mut().for_each(|particle| {
        particle.update(&app.mouse, &model.modifier);
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    // Draw a transparent black rectangle to make the particles fade.
    draw.rect()
        .w_h(app.main_window().rect().w() as f32, app.main_window().rect().h() as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.05));

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
