#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Note {
    pub midi_num: u8,
    pub duration: u64,
    pub velocity: u8,
}

pub struct Scale {
    steps: Vec<u8>,
}

impl Note {
    pub fn new(midi_num: u8, duration: u64, velocity: u8) -> Self {
        Self {
            midi_num,
            duration,
            velocity,
        }
    }
}

impl Scale {
    pub fn new(steps: Vec<u8>) -> Self {
        Self { steps }
    }

    pub fn next_from(&self, degree: i32) -> u8 {
        self.steps[degree.rem_euclid(self.steps.len() as i32) as usize]
    }

    pub fn prev_from(&self, degree: i32) -> u8 {
        self.next_from(degree - 1)
    }
}
