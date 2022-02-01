use crate::accords;
use fraction::ToPrimitive;

#[derive(Debug, Clone)]
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
        self.accords.push(accord)
    }

    pub fn sequence_from(accords: &[accords::Accord], volume: fraction::Fraction) -> Option<std::vec::Vec<Tact>>
    {
        let mut notes_durations = accords[0].first.unwrap().duration;
        for accord in accords.iter().skip(1)
        {
            notes_durations += accord.first.unwrap().duration;
        }

        if (notes_durations % volume).to_i8() != Some(0i8)
        {
            return None;
        }

        // TODO Place this logic into add_accord and add current_volume to Tact

        let mut tacts = std::vec::Vec::new();
        let mut current_tact = Tact::new(volume);
        let mut current_tact_volume = crate::Duration::nan();

        for accord in accords.iter()
        {
            if current_tact_volume == volume
            {
                tacts.push(current_tact);
                current_tact_volume = crate::Duration::nan();
                current_tact = Tact::new(volume);
            }

            current_tact.add_accord(*accord);
            if current_tact_volume.is_nan()
            {
                current_tact_volume = accord.first.unwrap().duration;
            }
            else
            {
                current_tact_volume += accord.first.unwrap().duration;
            }
        }   

        Some(tacts)
    }
}

impl std::fmt::Display for Tact
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Tact [\n{}\n]\n", self.accords.iter().map(|accord| accord.to_string()).collect::<std::vec::Vec<String>>().join("\n"))
    }

}
