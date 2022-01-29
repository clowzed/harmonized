

lazy_static!{
    static ref AVAILABLE_NOTES_DURATIONS: std::vec::Vec<fraction::Fraction> = vec![fraction::Fraction::new(8_u64, 1_u64),
                                                                                   fraction::Fraction::new(4_u64, 1_u64),
                                                                                   fraction::Fraction::new(2_u64, 1_u64),
                                                                                   fraction::Fraction::new(1_u64, 1_u64),
                                                                                   fraction::Fraction::new(1_u64, 2_u64),
                                                                                   fraction::Fraction::new(1_u64, 4_u64),
                                                                                   fraction::Fraction::new(1_u64, 8_u64),
                                                                                   fraction::Fraction::new(1_u64, 16_u64),
                                                                                   fraction::Fraction::new(1_u64, 32_u64),
                                                                                   fraction::Fraction::new(1_u64, 64_u64),
                                                                                   fraction::Fraction::new(1_u64, 128_u64),
                                                                                   fraction::Fraction::new(1_u64, 256_u64)
    ];
}

#[derive(Debug, Clone, Copy)]
pub struct NoteHeight
{
    octave: u8,
    number: u8,

}

#[derive(Debug, Clone, Copy)]
pub struct Note
{
    height: NoteHeight,
    duration: fraction::Fraction,
}

impl NoteHeight
{
    pub fn new(octave: u8, number: u8) -> NoteHeight
    {
        //? Checking if number is too big for octave
        NoteHeight{octave: octave + number / 7, number: number + number % 7}
    }
}

impl Note
{
    pub fn new(height: NoteHeight, duration: fraction::Fraction) -> Note
    {
        if AVAILABLE_NOTES_DURATIONS.contains(&duration)
        {
            return Note{height, duration}
        }
        panic!("Incoorect duration of note! Got: {}. Available: {}", 
                duration, 
                AVAILABLE_NOTES_DURATIONS.iter()
                                         .map(|duration| 
                                               duration.to_string())
                                         .collect::<Vec<String>>()
                                         .join(", "));
    }
}

impl std::fmt::Display for Note
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Note: height = [octave = {}, number = {}], duration = {}", self.height.octave, self.height.number, self.duration)
        
    }
}