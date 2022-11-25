use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::mpsc::Sender;

pub fn init(tx: Sender<Vec<u8>>) -> Option<MidiInputConnection<()>> { 
    let mut midi_in = match MidiInput::new("reading input") {
        Err(_) => panic!(),
        Ok(midi_input) => midi_input,
    };
    midi_in.ignore(Ignore::None);
    let in_port = midi_in
        .ports()
        .iter()
        .find(|&p| midi_in.port_name(p).unwrap() == dotenv!("MIDI_DEVICE"))
        .cloned();
    
    match in_port {
        None => {
            println!("No midi port :(");
            None
        }
        Some(input_port) => {
            match midi_in.connect(
                &input_port,
                "midir-read-input",
                move|_stamp, message, _| {
                    tx.send(Vec::from(message)).unwrap();
                },
                (),
            ) {
                Err(_) => None,
                Ok(connection) => Some(connection),
            }
        }
    }
}