use std::fmt::Result;
use crate::notes;


#[derive(Debug, Clone, Copy)]
pub struct Accord
{
    pub first  : Option<notes::Note>,
    pub second : Option<notes::Note>,
    pub third  : Option<notes::Note>,
    pub fourth : Option<notes::Note>,
}

impl Accord
{
    pub fn new() -> Accord
    {
        Accord{first: None, second: None, third: None, fourth: None}
    }

    pub fn set_note(&mut self, new_note: Option<notes::Note>, position: u8) -> bool
    {
        match position
        {   
            1 => self.first   = new_note,
            2 => self.second  = new_note,
            3 => self.third   = new_note,
            4 => self.fourth  = new_note,
            _ => return false,
        }
        true
    }

    pub fn get_note(&self, position: u8) -> Option<notes::Note>
    {
        match position
        {   
            1 => self.first,
            2 => self.second,
            3 => self.third,
            4 => self.fourth,
            _ => None,
        }
    }
}

impl std::fmt::Display for Accord
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> self::Result 
    {
        writeln!(f, "Accord [\n{}\n{}\n{}\n{}\n]\n", 
        match self.first.is_some()  {true=> self.first.unwrap().to_string(),  false => "Note: not setted".to_string()},
        match self.second.is_some() {true=> self.second.unwrap().to_string(), false => "Note: not setted".to_string()},
        match self.third.is_some()  {true=> self.third.unwrap().to_string(),  false => "Note: not setted".to_string()},
        match self.fourth.is_some() {true=> self.fourth.unwrap().to_string(), false => "Note: not setted".to_string()},
    )
    }
}