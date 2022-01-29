

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
        let new_number = match number % 7 {0 => 7, _ => number % 7};
        let new_octave = match number % 7 {0 => octave + number / 7 - 1, _ => octave + number / 7};
        
        //? Checking if number is too big for octave
        NoteHeight{octave: new_octave, number: new_number}
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

#[cfg(test)]
mod notes_tests 
{
    #[test]
    fn test_octave_change() 
    {
        let height = crate::notes::NoteHeight::new(1, 1);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 1);
        let height = crate::notes::NoteHeight::new(1, 2);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 2);
        let height = crate::notes::NoteHeight::new(1, 3);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 3);
        let height = crate::notes::NoteHeight::new(1, 4);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 4);
        let height = crate::notes::NoteHeight::new(1, 5);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 5);
        let height = crate::notes::NoteHeight::new(1, 6);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 6);
        let height = crate::notes::NoteHeight::new(1, 7);
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 7);
        let height = crate::notes::NoteHeight::new(1, 8);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 1);
        let height = crate::notes::NoteHeight::new(1, 9);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 2);
        let height = crate::notes::NoteHeight::new(1, 10);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 3);
        let height = crate::notes::NoteHeight::new(1, 11);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 4);
        let height = crate::notes::NoteHeight::new(1, 12);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 5);
        let height = crate::notes::NoteHeight::new(1, 13);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 6);
        let height = crate::notes::NoteHeight::new(1, 14);
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 7);
        let height = crate::notes::NoteHeight::new(1, 15);
        assert_eq!(height.octave, 3);
        assert_eq!(height.number, 1);   
    }
}