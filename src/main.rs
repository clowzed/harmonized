mod notes;
mod accords;
mod tacts;
use text_io::read;


fn main() -> Result<(), fraction::error::ParseError>
{
    let accords_amount = 8;
    let tacts_volume = fraction::Fraction::new(1 as u64, 1 as u64);

    let mut current_tact = tacts::Tact::new(tacts_volume);
    let mut accords_durations_sum = fraction::Fraction::new(0_u64, 0_u64);

    let mut all_tacts = std::vec::Vec::new();

    for _ in 0..accords_amount
    {
        println!("accords_durations_sum: {}", accords_durations_sum);
        if !accords_durations_sum.is_nan() && accords_durations_sum >= tacts_volume
        {
            all_tacts.push(current_tact);
            current_tact = tacts::Tact::new(tacts_volume);
            accords_durations_sum = fraction::Fraction::new(0_u64, 0_u64);
        }

        let mut new_accord = accords::Accord::new();

        //? Reading note params
        println!("Enter params for the fist note of accord");

        println!("Note height - octave : ");
        let octave:u8 = read!();

        println!("Note height - number : ");
        let number: u8 = read!();
        println!("{}", number);

        let note_height = notes::NoteHeight::new(octave, number);
        
        println!("Note duration: ");
        let duration:String = read!();
        let duration = fraction::Fraction::from_decimal_str(&duration)?;
        println!("duration: {}", duration);

        let new_note = notes::Note::new(note_height, duration);

        new_accord.set_note(Some(new_note), 1);

        current_tact.add_accord(new_accord);

        if accords_durations_sum.is_nan()
        {
            accords_durations_sum = duration;
        }
        else 
        {
            accords_durations_sum += duration;      
        }
    }

    for tact in all_tacts.iter()
    {
        println!("{}", tact);
    }

    Ok(())
}
