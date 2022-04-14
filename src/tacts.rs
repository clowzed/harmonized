use crate::accords;
use fraction::ToPrimitive;

<<<<<<< HEAD
pub enum AddResult
{
    WAS_ADDED,
    IS_FULL,
    INCORRECT_VOLUME,
}
    
=======
#[derive(Debug, Clone)]
>>>>>>> d7cccd8e17a7c48ccf287ed8fbd79e467eab9bae
pub struct Tact
{
    accords: std::vec::Vec<accords::Accord>,
    volume: fraction::Fraction,
}

impl Tact
{
    pub fn new(volume: fraction::Fraction) -> Tact
    {
        Tact {accords : std::vec::Vec::new(), volume}
    }

    pub fn get_accord(&self, position: usize) -> Option<&accords::Accord>
    {
        self.accords.get(position)
    }

    pub fn add_accord(&mut self, accord: accords::Accord)
    {
        if self.is_full()
        {
            return AddResult::IS_FULL;
        }
        
        let inner_volume = 
        self.accords.push(accord)
    }
    
    pub fn is_full(&self)
    {
        return volume == self.accords.iter().map(|accord| accord.first_note.unwrap()).sum();
    }
    
}

impl std::fmt::Display for Tact
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Tact [\n{}\n]\n", self.accords.iter().map(|accord| accord.to_string()).collect::<std::vec::Vec<String>>().join("\n"))
    }

}
