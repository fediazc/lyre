use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Note {
    pub midi_num: u8,
    pub duration: u64,
    pub velocity: u8,
}

impl Note {
    pub fn new(midi_num: u8, duration: u64, velocity: u8) -> Self {
        Self {
            midi_num,
            duration,
            velocity,
        }
    }

    pub fn get_name(&self) -> String {
        let oct = self.midi_num as i32 / 12 - 1;
        let base_midi_num = self.midi_num as i32 - 12 * oct - 12;
        let note_name = match base_midi_num {
            0 => "C",
            1 => "Db",
            2 => "D",
            3 => "Eb",
            4 => "E",
            5 => "F",
            6 => "Gb",
            7 => "G",
            8 => "Ab",
            9 => "A",
            10 => "Bb",
            11 => "B",
            _ => todo!(),
        };

        format!("{note_name}{oct}")
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
