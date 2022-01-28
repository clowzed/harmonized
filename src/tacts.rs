use crate::accords;

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
}

impl std::fmt::Display for Tact
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Tact [\n{}\n]\n", self.accords.iter().map(|accord| accord.to_string()).collect::<std::vec::Vec<String>>().join("\n"))
    }

}
