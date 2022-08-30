use rand::{random, thread_rng, Rng};
use rand_derive2::RandGen;
use strum_macros::{Display};

use crate::section::Section;

#[derive(Debug, RandGen, Display)]
enum Name {
    Akira,
    Californa,
    Daedalus,
    Eisenberg,
    Interpid,
    Miranda,
    Nova,
    Reliant,
    Segan,
}

#[derive(Debug, RandGen)]
pub struct Station {
    name: Name,
    version: u8,
    pub sections: Vec<Section>,
}

impl Station {
    pub fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            sections: (0..10).map(|_| random()).collect(),
        }
    }

    pub fn days_left(&self) -> usize {
        self.sections.iter().filter(|m| m.active)
        .count()
    }

    pub fn working_sections(&self) -> Vec<String> {
        self.sections.iter().filter(|m| m.active)
        .map(|m| m.name.to_string())
        .collect()
    }

    pub fn broken_sections(&self) -> Vec<String> {
        self.sections.iter().filter(|m| !m.active)
        .map(|m| m.name.to_string())
        .collect()
    }
    pub fn new_day(&mut self) {
        self.break_something();
    }

    pub fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-Failure {}", &broken_section.name);
        } else {
            println!("(Sections OK)");
        }
    }

    pub fn status (&self) {
        dbg!(&self); 
    }
}
