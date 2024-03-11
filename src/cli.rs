use clap::{Args, ValueEnum};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the input file
    pub file: String,
    /// The number of iterations to perform on the L-system
    #[arg(short, long)]
    pub depth: u32,
    /// Path for the output MIDI file
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
