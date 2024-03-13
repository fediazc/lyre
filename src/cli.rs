use clap::{Args, ValueEnum};

#[derive(clap::Parser)]
#[command(
    version,
    about = "Make music with L-systems",
    long_about = LONG_ABOUT
)]
pub struct Cli {
    /// The path to the input file
    pub file: String,
    /// The number of iterations to perform on the L-system
    #[arg(short, long)]
    pub depth: u32,
    /// The file path to output the MIDI file to
    #[arg(short, long)]
    pub out: String,
    /// The scale type to use
    #[command(flatten)]
    pub scale_opts: ScaleOpts,
    /// The starting note to play, as a MIDI key number between 0 and 127. Middle C is key 60.
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=127), default_value_t = 60)]
    pub start_at: u8,
    /// Print the resulting L-system sequence.
    #[arg(short, long)]
    pub print_result: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct ScaleOpts {
    /// The scale type to use
    #[arg(name="scale", short, long, value_enum, default_value_t=ScaleType::Major)]
    pub scale_type: ScaleType,

    /// Define the scale to use, as a list of integers. E.g. the major scale would be
    /// 2,2,1,2,2,2,1 (1=half-step, 2=whole-step)
    ///
    /// Each integer on the list represents the number of half-steps required to reach the next
    /// note in the scale. E.g. the minor scale is commonly represented as W-H-W-W-H-W-W
    /// (W = whole-step = 2 half-steps) which translates to 2,1,2,2,1,2,2. You can also try creating
    /// 'weird' scales with more than a whole step between some notes, and with less (or more) than
    /// 7 notes in total, such as 1,3,3,3,2.
    #[arg(long, num_args=1.., value_delimiter=',')]
    pub custom_scale: Option<Vec<u8>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ScaleType {
    Chromatic,
    Major,
    Minor,
}

static LONG_ABOUT: &str = "The following describes how to write a valid L-system file:

    - A symbol can be any uppercase letter, or any of the following special characters: '[', ']', '+', '-'.
    - The syntax for a rule is 'A => B' where 'A' is a single symbol and 'B' is a sequence of symbols. For example 'S => SS' and 'X => S+[X]-X' are both valid rules.
    - A valid input file is a text file containing a list of rules, each on a separate line, followed by a sequence of symbols defining the axiom. The order of the rules does NOT affect the final result, but the axiom must always come after the list of rules.
    - Anything written after a '#' character is considered a comment and is ignored.

For example, the following is a valid L-system definition:


        S => SS      # rule 1
        X => S+[X]-X # rule 2

        X            # axiom

In this example, 'X' is the axiom.

To generate music, the resulting string from the L-system is read from left to right. The characters 'S', '[', ']', '+', '-' are special symbols which perform the following actions:

    - 'S': Play a sixteenth note. Multiple consecutive 'S's are played as a single note, with the length of the note matching the number of 'S's. For example, 'SS' will play a single note with the length of two sixteenth notes, a.k.a an eighth note, and 'SSSS' will play a quarter note.
    - '+': Move the note to be played UP by a step defined by the scale. For example, 'S+S' will play C and then C#.
    - '-': Move the note to be played DOWN by a step defined by the scale. For example, 'S-S' will play C and then B.
    - '[': Push the current state into the stack. The state consists of simply the note to be played.
    - ']': Pop the state. For example, 'S[+S]S' will play C, then C#, and finally C again.

The examples above start from the note C and use the chromatic scale. You can change the starting note with the `--start-at` option, and the scale with the `--scale` or `--custom-scale` options (by default the major scale will be used).
";
