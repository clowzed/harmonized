

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
        NoteHeight{ octave, number}
    }
}

impl Note
{
    pub fn new(height: NoteHeight, duration: fraction::Fraction) -> Note
    {
        Note{height, duration}
    }
}

impl std::fmt::Display for Note
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Note: height = [octave = {}, number = {}], duration = {}", self.height.octave, self.height.number, self.duration)
        
    }
}