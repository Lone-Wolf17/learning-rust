use rand_derive2::RandGen;
use strum_macros::{Display, EnumString};


#[derive(Debug, RandGen, Eq, PartialEq)]
pub struct Section {
    pub name: SectionName,
    pub active: bool,
}

#[derive(Debug, RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
pub enum SectionName {
    AstroScience, Solar, Antenna,
    RadiationMirrors, Sleeping, NuclearGenerator,
    Galley, Transponder, Tracking
}
