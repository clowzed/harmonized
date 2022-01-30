//? Here we define all available notes durations
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

const OCTAVE_MAX_NUMBER: crate::Octave = 8;
const MIN_NUMBER: crate::Number = 1;
const MAX_NUMBER: crate::Number = 7 * (OCTAVE_MAX_NUMBER as crate::Number);





#[derive(Debug, Clone, Copy)]
pub struct NoteHeight
{
    octave : crate::Octave,
    number : crate::Number,

}

#[derive(Debug, Clone, Copy)]
pub struct Note
{
    height   : NoteHeight,
    duration : crate::Duration,
}





impl NoteHeight
{
    pub fn new(octave: crate::Octave, number: crate::Number) -> Option<NoteHeight>
    {
        return match NoteHeight::correct_parameters(octave, number)
        {
            true  => Some(NoteHeight{octave, number}.normalized()),
            false => None,
        };        
    }
    
    
    pub fn correct_parameters(octave: crate::Octave, number: crate::Number) -> bool
    {
        let concret_number = NoteHeight::concret_number(octave, number);
        MIN_NUMBER <= concret_number && concret_number<= MAX_NUMBER
    }
    
    pub fn concret_number(octave:crate::Octave, number: crate::Number) -> crate::Number 
    {   
        (octave * 7) as crate::Number + number
    }
    
    
    // ! No check for parameters
    // ! Call 'correct_parameters' first
    pub fn normalized(&self) -> Self
    {
        let numbers_before = (self.octave * 7) as crate::Number;
        let current_number = numbers_before + self.number;
        
        let new_number = match current_number % 7 
                            {
                                0 => 7,
                                _ => current_number % 7
                            };
        
        let new_octave = match current_number % 7 
                            {   
                                0 => ((current_number / 7) - (1 as crate::Number)) as crate::Octave,
                                _ => (current_number / 7)  as crate::Octave,
                            };
        
        NoteHeight{octave: new_octave, number: new_number}
    }
}








impl Note
{
    pub fn new(height: NoteHeight, duration: crate::Duration) -> Option<Note>
    {
        return match AVAILABLE_NOTES_DURATIONS.contains(&duration)
        {
            true  => Some(Note{height, duration}),
            false => None,
        };
    }
}



impl std::fmt::Display for Note
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Note: height = [octave = {}, number = {}], duration = {}", 
                  self.height.octave, 
                  self.height.number, 
                  self.duration)
        
    }
}








#[cfg(test)]
mod notes_tests 
{
    #[test]
    fn test_octave_change() 
    {
        // ? Checking positive
        let height = crate::notes::NoteHeight::new(1, 1).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 1);
        let height = crate::notes::NoteHeight::new(1, 2).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 2);
        let height = crate::notes::NoteHeight::new(1, 3).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 3);
        let height = crate::notes::NoteHeight::new(1, 4).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 4);
        let height = crate::notes::NoteHeight::new(1, 5).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 5);
        let height = crate::notes::NoteHeight::new(1, 6).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 6);
        let height = crate::notes::NoteHeight::new(1, 7).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 7);
        let height = crate::notes::NoteHeight::new(1, 8).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 1);
        let height = crate::notes::NoteHeight::new(1, 9).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 2);
        let height = crate::notes::NoteHeight::new(1, 10).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 3);
        let height = crate::notes::NoteHeight::new(1, 11).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 4);
        let height = crate::notes::NoteHeight::new(1, 12).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 5);
        let height = crate::notes::NoteHeight::new(1, 13).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 6);
        let height = crate::notes::NoteHeight::new(1, 14).unwrap();
        assert_eq!(height.octave, 2);
        assert_eq!(height.number, 7);
        let height = crate::notes::NoteHeight::new(1, 15).unwrap();
        assert_eq!(height.octave, 3);
        assert_eq!(height.number, 1);
        let height = crate::notes::NoteHeight::new(2, -4).unwrap();
        assert_eq!(height.octave, 1);
        assert_eq!(height.number, 3);  
    }

    #[test]
    fn test_params_check()
    {
        let height = crate::notes::NoteHeight::new(0, -1);
        assert!(height.is_none());

        let height = crate::notes::NoteHeight::new(1, 1);

        assert!(height.is_some());
        let height = crate::notes::NoteHeight::new(9, 31);
        assert!(height.is_none());
    }
}
