#[macro_use]
extern crate lazy_static;

mod notes;
mod accords;
mod tacts;
use dialoguer::{theme::ColorfulTheme, Select};


type Octave   = u8;
type Number   = i8;
type Duration = fraction::Fraction;


#[derive(Debug)]
struct Configuration
{
    accords_amount : u8,
    tacts_volume   : fraction::Fraction,
    
}


fn split_accords_into_tacts(config: &Configuration, accords: &Vec<accords::Accord>) -> Option<Vec<tacts::Tact>>
{
    let tacts_list = vec![];
    
    let tact = tacts::Tact::new(config.tacts_volume);
    
    for accord in accord.iter()
    {
        match tact.add_accord(accord)
        {
            tacts::AddResult::WAS_ADDED => {},
            tact::AddResult::IS_FULL
            {
                tacts.push(tact);
                tact = tacts::Tact::new(config.tacts_volume);
            },
            tact::AddResult::INCORRECT_VOLUME => {
                return None;
            }
        }
    }
    
    Some(tacts);
}


fn get_note(config: &Configuration) -> Option<notes::Note>
{
    let octaves:Vec<u8> = (1u8..config.accords_amount + 1).collect();
    let numbers:Vec<i8> = (1i8..8i8).collect();

    
    let selection = Select::with_theme(&ColorfulTheme::default())
                                 .with_prompt("Pick octave number")
                                 .default(0)
                                 .items(&octaves[..])
                                 .interact()
                                 .unwrap();
    
    let octave = octaves[selection];
    
    
    let selection = Select::with_theme(&ColorfulTheme::default())
                                 .with_prompt("Pick note number")
                                 .default(0)
                                 .items(&numbers[..])
                                 .interact()
                                 .unwrap();
    
    let number = numbers[selection];
    
    let note_height = match notes::NoteHeight::new(octave, number)
    {
        Some(height) => height,
        None => return None,
    };
    
    
    let selection = Select::with_theme(&ColorfulTheme::default())
                                 .with_prompt("Pick note duration")
                                 .default(0)
                                 .items(&crate::notes::AVAILABLE_NOTES_DURATIONS[..])
                                 .interact()
                                 .unwrap();
    
    let duration = crate::notes::AVAILABLE_NOTES_DURATIONS[selection];
    
    match duration <= config.tacts_volume
    {
        true => {},
        false => return None,
    }
    
    let note = match notes::Note::new(note_height, duration)
    {
        Some(note) => note,
        None => return None
    };
    
    Some(note)
    
}


fn main()
{
    let config = Configuration 
    {
        accords_amount : 8,
        tacts_volume   : fraction::Fraction::new(1u16, 1u16),
    };
    
    
    
    let mut first_notes = std::vec::Vec::new();
    
    
    
    for _ in 0..config.accords_amount
    {
        let new_note = match get_note(&config)
        {
            Some(note) => note,
            None => 
            {
                println!("Failed to create note with this params!");
                return;
            }
        };
        first_notes.push(new_note);
    }
    
    

    let accords: Vec<accords::Accord> = first_notes.iter()
                                                   .map(|note| 
                                                        {
                                                            let mut new_accord = accords::Accord::new();
                                                            new_accord.set_note(Some(*note), 1);
                                                            new_accord
                                                        })
                                                   .collect();
    
    let tacts = match tacts::Tact::sequence_from(&accords, config.tacts_volume)
    {
        println!("{}", accord);
    }
    
    
    let tacts = split_accords_into_tacts(&accords);
    
    for tact in tacts.iter()
    {
        println!("{}", tact);
    }

}
