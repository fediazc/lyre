mod cli;
mod lsystem;
mod note;
mod parser;

use clap::Parser;
use midly::{Format, Header, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind};

use crate::cli::{Cli, ScaleType};
use crate::note::Scale;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let scale = match cli.scale_opts.custom_scale {
        Some(cs) => Scale::new(cs),
        None => match cli.scale_opts.scale_type {
            ScaleType::Chromatic => Scale::new(vec![1]),
            ScaleType::Major => Scale::new(vec![2, 2, 1, 2, 2, 1]),
            ScaleType::Minor => Scale::new(vec![2, 1, 2, 2, 1, 2, 2]),
        },
    };

    let header = Header::new(Format::SingleTrack, Timing::Metrical(4.into()));
    let mut smf = Smf::new(header);

    println!("Parsing file {}...", cli.file);
    let mut sys = crate::parser::parse_file(&cli.file)?;

    println!("Generating sequence...");
    sys.forward(cli.depth);

    println!("Creating MIDI file...");
    let mut events: Track = vec![];
    for note in sys.get_notes(scale, cli.start_at) {
        let on_event = note_on(note.midi_num, note.velocity);
        let off_event = note_off(note.duration as u32, note.midi_num, note.velocity);

        events.push(on_event);
        events.push(off_event);
    }

    smf.tracks = vec![events];

    if let Err(e) = smf.save(&cli.out) {
        return Err(format!(
            "could not save MIDI file to path {}: {}",
            cli.out, e
        ));
    }

    println!("Done");

    if cli.print_result {
        println!("Result: {sys}");
    }

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
