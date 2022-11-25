use midir::MidiInputConnection;
use nannou::prelude::*;
use particles::{RotatingParticle, Modifier};
use std::{sync::mpsc::{channel, Receiver}, collections::HashMap};
use wmidi::{ControlFunction, MidiMessage, Note};

#[macro_use]
extern crate dotenv_codegen;

pub mod midi;
pub mod particles;

fn main() {
    nannou::app(model).update(update).run();
}

const MAX_RADIUS_SCALE: f32 = 5.;
const MAX_ACCELERATOR: f32 = 5.;
const MIDI_MAX_VALUE: u8 = 127;
const ROTATING_MIN_NOTE: u8 = 48;
const ROTATION_MAX_NOTE: u8 = 72;

struct Model {
    rotating_particles: HashMap<Note, RotatingParticle>,
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

    let (tx, rx) = channel();

    Model {
        rotating_particles: HashMap::new(),
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
                    // CC control modifiers
                    wmidi::MidiMessage::ControlChange(_, control, value) => {
                        if control == ControlFunction::MODULATION_WHEEL {
                            model.modifier.scale = 1.0 + (MAX_RADIUS_SCALE - 1.0)
                                * <u8 as From<wmidi::U7>>::from(value) as f32
                                / (MIDI_MAX_VALUE as f32);
                        } else if control == ControlFunction::SOUND_CONTROLLER_5 {
                            model.modifier.accelerator = 1.0 + (MAX_ACCELERATOR - 1.0)
                                * <u8 as From<wmidi::U7>>::from(value) as f32
                                / (MIDI_MAX_VALUE as f32);
                        }
                    }
                    // Notes add new shapes to the scene.
                    wmidi::MidiMessage::NoteOn(_channel, note, velocity) => {
                        let w_h = app.main_window().rect().w_h();
                        let note_value = <u8 as From<Note>>::from(note);
                        let vel_value = <u8 as From<wmidi::U7>>::from(velocity);
                        let x: f32;
                        let y: f32;
                        match note_value {
                            ROTATING_MIN_NOTE..=ROTATION_MAX_NOTE => {
                                let slope = w_h.0 / (ROTATION_MAX_NOTE as f32 - ROTATING_MIN_NOTE as f32);
                                x = slope * (note_value as f32 - ROTATING_MIN_NOTE as f32) - w_h.0 / 2.;
                                y = w_h.1 / 2. * (vel_value as f32) / (MIDI_MAX_VALUE as f32) - w_h.1 / 2.;
                            },
                            _ => {
                                x = w_h.0 * (note_value as f32) / (MIDI_MAX_VALUE as f32) - w_h.0 / 2.;
                                y = - w_h.1 / 2. * (vel_value as f32) / (MIDI_MAX_VALUE as f32) + w_h.1 / 2.;
                            }
                        }
                        
                        model.rotating_particles.insert(note, RotatingParticle::new(vec2(x, y), vel_value));
                    }
                    wmidi::MidiMessage::NoteOff(_channel, note, _velocity ) => {
                        model.rotating_particles.get_mut(&note).unwrap().held = false; // Start fading when the note is released.
                    }
                    _ => (),
                }
            }
        };
    }

    // Remove the particles that have expired their lifetime.
    model.rotating_particles.retain(|_, particle| particle.lifetime > 0);

    // Add some variation depending on time
    model.modifier.center_shift = vec2(app.time.cos() * 10.0, app.time.sin() * 10.0);
    model.rotating_particles.iter_mut().for_each(|particle| {
        particle.1.update(&model.modifier);
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

    // Draw the particles
    model.rotating_particles.iter().for_each(|particle| {
        particle.1.draw(&draw);
    });

    draw.to_frame(app, &frame).unwrap();
}
