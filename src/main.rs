use log::{debug, error, info, warn};


#[macro_use]
extern crate lazy_static;

mod notes;
mod accords;
mod tacts;
use text_io::read;


fn main() -> Result<(), fraction::error::ParseError>
{

    dotenv::dotenv().ok();
    env_logger::init();
    
    //? Config
    let accords_amount = 8;
    let tacts_volume = fraction::Fraction::new(1 as u64, 1 as u64);
    
    debug!("Accords amount: {}", accords_amount);
    debug!("Tacts   volume: {}", tacts_volume);



    //? Temp variables
    let mut current_tact = tacts::Tact::new(tacts_volume);
    let mut accords_durations_sum = fraction::Fraction::new(0_u64, 0_u64);



    let mut all_tacts = std::vec::Vec::new();



    for accord_index in 0..accords_amount
    {
        debug!("filling accord with number: {}", accord_index);
        
        //? checking if tact is full
        if !accords_durations_sum.is_nan() && accords_durations_sum >= tacts_volume
        {
            debug!("Tact volume is full! Creating new tact.");
            
            all_tacts.push(current_tact);
            current_tact = tacts::Tact::new(tacts_volume);
            accords_durations_sum = fraction::Fraction::new(0_u64, 0_u64);
        }

        let mut new_accord = accords::Accord::new();





        //? Reading note params
        println!("Enter params for the fist note of accord");

        println!("Note height - octave : ");
        let octave:u8 = read!();
        
        debug!("Entered octave: {}", octave);

        println!("Note height - number : ");
        let number: u8 = read!();
        debug!("Entered number: {}", number);

        let note_height = notes::NoteHeight::new(octave, number);
        debug!("Created note height: {:?}", note_height);
        
        
        println!("Note duration: ");
        let duration:String = read!();
             
    
        let duration = fraction::Fraction::from_decimal_str(&duration)?;

        let new_note = notes::Note::new(note_height, duration);




        
        //? Setting first node in accord
        new_accord.set_note(Some(new_note), 1);


        current_tact.add_accord(new_accord);



        // ? Guard of notes sum of durations!
        if accords_durations_sum.is_nan()
        {
            accords_durations_sum = duration;
        }
        else 
        {
            accords_durations_sum += duration;      
        }

        if accords_durations_sum > tacts_volume
        {
            panic!("Volume of tact is smaller than enterd new note duration! Durations of first notes: {}. Tact volume: {}", accords_durations_sum, tacts_volume);
        }
    }



    for tact in all_tacts.iter()
    {
        println!("{}", tact);
    }


    Ok(())
}
