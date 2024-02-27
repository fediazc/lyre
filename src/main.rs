mod lsystem;
mod note;
mod parser;

use midly::{Format, Header, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind};
use std::env;

use crate::note::Scale;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("no filename given");
    }

    let fname = &args[1];

    let header = Header::new(Format::SingleTrack, Timing::Metrical(4.into()));
    let mut smf = Smf::new(header);

    println!("Parsing file {}...", fname);
    let mut sys = crate::parser::parse_file(fname)?;

    println!("Generating sequence...");
    sys.forward(3);

    println!("Creating MIDI file...");
    let mut events: Track = vec![];
    for note in sys.get_notes(Scale::new(vec![2, 1, 2, 2, 1, 2, 2])) {
        let on_event = note_on(note.midi_num, note.velocity);
        let off_event = note_off(note.duration as u32, note.midi_num, note.velocity);

        events.push(on_event);
        events.push(off_event);
    }

    smf.tracks = vec![events];

    if smf.save("example.mid").is_err() {
        return Err("could not save midi file");
    }

    println!("Done");

    Ok(())
}

fn note_on(key: u8, vel: u8) -> TrackEvent<'static> {
    TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Midi {
            channel: 1.into(),
            message: MidiMessage::NoteOn {
                key: key.into(),
                vel: vel.into(),
            },
        },
    }
}

fn note_off(delta: u32, key: u8, vel: u8) -> TrackEvent<'static> {
    TrackEvent {
        delta: delta.into(),
        kind: TrackEventKind::Midi {
            channel: 1.into(),
            message: MidiMessage::NoteOff {
                key: key.into(),
                vel: vel.into(),
            },
        },
    }
}
