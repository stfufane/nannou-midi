use midir::MidiInputConnection;
use nannou::prelude::*;
use particles::{Particle, Modifier};
use std::{sync::mpsc::{channel, Receiver}, collections::HashMap};
use wmidi::{ControlFunction, MidiMessage, U7, Note};

pub mod midi;
pub mod particles;

fn main() {
    nannou::app(model).update(update).run();
}

const NB_PARTICLES: usize = 50;
const MAX_RADIUS_SCALE: f32 = 10.0;
const MAX_ACCELERATOR: f32 = 5.0;

struct Model {
    particles: Vec<Particle>,
    notes: HashMap<Note, Point2>,
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
        let particle = Particle::new(&app.window_rect());
        particles.push(particle);
    }

    let (tx, rx) = channel();

    Model {
        particles,
        notes: HashMap::new(),
        modifier: Modifier::new(),
        _connection: midi::init(tx),
        receiver: rx,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for data in model.receiver.try_iter() {
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
                    wmidi::MidiMessage::NoteOn(_channel, note, _velocity) => {
                        let width = app.main_window().rect().w();
                        let note_value = <u8 as From<Note>>::from(note) as f32;
                        let note_max = <u8 as From<Note>>::from(Note::HIGHEST_NOTE) as f32;
                        let x = width * note_value / note_max - width / 2.;
                        model.notes.insert(note, vec2(x, - app.main_window().rect().h() / 2.));
                    }
                    wmidi::MidiMessage::NoteOff(_channel, note, _velocity ) => {
                        model.notes.remove_entry(&note);
                    }
                    _ => (),
                }
            }
        };
    }

    // Add some variation depending on time
    model.modifier.center_shift = vec2(app.time.cos() * 10.0, app.time.sin() * 10.0);
    model.particles.iter_mut().for_each(|particle| {
        particle.update(&model.modifier);
    });
    model.notes.iter_mut().for_each(|note| { 
        note.1.y += 9.;
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
        particle.draw(&draw);
    });

    model.notes.iter().for_each(|note| {
        draw.ellipse()
            .x_y(note.1.x, note.1.y)
            .radius(12.)
            .color(nannou::color::LIME);
    });

    draw.to_frame(app, &frame).unwrap();
}
