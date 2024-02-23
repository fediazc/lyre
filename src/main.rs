mod lsystem;
mod note;
mod parser;

use midly::{Format, Header, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind};
use note::Note;
use std::{collections::HashSet, env};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("no filename given");
    }

    let fname = &args[1];

    let header = Header::new(Format::SingleTrack, Timing::Metrical(4.into()));
    let mut smf = Smf::new(header);

    println!("Parsing file {}...", fname);
    let mut system = crate::parser::parse_file(fname)?;

    println!("Generating sequence...");
    system.forward(4);

    println!("Creating MIDI file...");
    let mut events: Track = vec![];
    let mut bad_notes: HashSet<u8> = HashSet::new();
    for note in system
        .elements
        .iter()
        .map(|e| e.note)
        .collect::<Vec<Note>>()
    {
        if note.midi_num > 127 {
            bad_notes.insert(note.midi_num);
        }

        let on_event = note_on(note.midi_num, note.velocity);
        let off_event = note_off(note.duration as u32, note.midi_num, note.velocity);

        events.push(on_event);
        events.push(off_event);
    }

    smf.tracks = vec![events];

    if bad_notes.len() > 0 {
        println!(
            "Warning: writing note(s) {}to a MIDI file will result in unxpected behavior. The maximum pitch allowed is G8 (The maximum MIDI key value is 127, equivalent to a G8 in this program).",
            bad_notes
                .iter()
                .map(|key| format!("{} ", note::name_from_key(*key as u32)))
                .collect::<String>()
        );
    }

    match smf.save("example.mid") {
        Err(_) => return Err("could not save midi file"),
        _ => (),
    }
    print!("Done\n");

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
