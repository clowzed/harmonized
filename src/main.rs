use log::{debug, error};


#[macro_use]
extern crate lazy_static;

mod notes;
mod accords;
mod tacts;
use text_io::read;

type Octave   = u8;
type Number   = i8;
type Duration = fraction::Fraction;


fn parse_duration(s: &str) -> Option<Duration> 
{
    lazy_static!{
        static ref FRACTION_PATTERN:regex::Regex = regex::Regex::new(r"^\d/\d$").unwrap();
    }
    return match FRACTION_PATTERN.is_match(s)
    {
        true => {
            let parts = s.split('/').collect::<Vec<_>>();
            let nominator:u64   = parts[0].parse().unwrap();
            let denominator:u64 = parts[1].parse().unwrap();
            Some(Duration::new(nominator, denominator))
        }
        false => None
    };
    
}

fn main()
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
        debug!("Filling accord with number: {}", accord_index);
        
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

        println!("Note height - octave : ");
        let octave:u8 = read!();
        
        debug!("Entered octave: {}", octave);

        println!("Note height - number : ");
        let number: i8 = read!();
        
        debug!("Entered number: {}", number);

        let note_height = match notes::NoteHeight::new(octave, number)
        {
            Some(height) => height,
            None => 
            {
                error!("Impossible to create such height!");
                std::process::exit(1);
            }
        };
        debug!("Created note height: {:?}", note_height);
        
        
        
        
        
        println!("Note duration: ");
        let duration:String = read!();
             
    
        let duration = match parse_duration(&duration)
        {
            Some(duration) => duration,
            None => {println!("Failed to parse duration: {}", duration); return;}
        };

        let new_note = match notes::Note::new(note_height, duration)
        {
            Some(note) => note,
            None => {
                error!("Couldn't create note!");
                std::process::exit(1);
            }
        };




        
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

}
