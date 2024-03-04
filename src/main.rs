mod lsystem;
mod note;
mod parser;

use clap::{Args, Parser, ValueEnum};
use midly::{Format, Header, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind};

use crate::note::Scale;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the input file
    file: String,
    /// The number of generations to perform on the L-system
    #[arg(short, long)]
    depth: u32,
    /// Path for the output MIDI file
    #[arg(short, long)]
    out: String,
    /// The scale type to use
    #[command(flatten)]
    scale_opts: ScaleOpts,
    /// The starting note to play, as a MIDI key number between 0 and 127. Middle C is key 60.
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=127), default_value_t = 60)]
    start_at: u8,
    /// Print the resulting L-system sequence.
    #[arg(short, long)]
    print_result: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct ScaleOpts {
    /// The scale type to use
    #[arg(name="scale", short, long, value_enum, default_value_t=ScaleType::Major)]
    scale_type: ScaleType,

    /// Define the scale to use, as a list of integers. E.g. the major scale would be
    /// 2,2,1,2,2,2,1 (1=half-step, 2=whole-step)
    ///
    /// Each integer on the list represents the number of half-steps required to reach the next
    /// note in the scale. E.g. the minor scale is commonly represented as W-H-W-W-H-W-W
    /// (W = whole-step = 2 half-steps) which translates to 2,1,2,2,1,2,2. You can also try creating
    /// 'weird' scales with more than a whole step between some notes, and with less (or more) than
    /// 7 notes in total, such as 1,3,3,3,2.
    #[arg(long, num_args=1.., value_delimiter=',')]
    custom_scale: Option<Vec<u8>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ScaleType {
    Chromatic,
    Major,
    Minor,
}

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
