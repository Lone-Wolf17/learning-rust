use std::str::FromStr;

use strum_macros::{EnumString, Display};
use inquire::{Select, Text};
use rand::{random, thread_rng, Rng};
use rand_derive2::RandGen;
fn main() {
    let mut station = Station::new();
    let mut station_log = vec![];

    loop { // main game loop
        let days_left = station.days_left();
        if days_left < 1 {
            println!("End-Transmission");
            break
        }

        println!("{days_left} Until final transmission");
        station_log.push(Text::new("Enter your log: ").prompt().unwrap());

        match menu(&[
            "NEW DAY".into(), "STATUS".into(), "POWERDOWN".into()
        ]).as_str() {
            "NEW DAY" => {
                station.new_day();
                match  menu(&["REPAIR".into(), "SCIENCE".into()]).as_str() {
                    "REPAIR" => {
                        repair(menu(&station.broken_sections()), &mut station);
                        continue;
                    },
                    "SCIENCE" => {
                        science(menu(&station.working_sections()), &mut station);
                        continue;
                    },
                    &_ => panic!(),
                }
            }
        ,
        "STATUS" => station.status(),
        "POWERDOWN" => break,
        &_ => panic!("Test")
    }

    }

    dbg!(station_log);
}

/// Build a simple meny based on 'items'
fn menu(items: &[String]) ->String {
    Select::new("Menu", items.to_vec()).prompt().unwrap()
}

// fixes 'broken_section' on a 'station'
fn repair(broken_section: String, station: &mut Station) {
    let section = SectionName::from_str(broken_section.as_str()).unwrap();

    let broken_index = station.sections.iter().position(|m| m.name == section).expect("Section not found.");

    let broken_section = station.sections.iter_mut().find(|m| m.name == section).expect("Section not found.");
    broken_section.active = true;
    station.sections[broken_index].active = true;
}

fn science (working_section: String, station: &mut Station) {
    station.break_something();
}

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
struct Station {
    name: Name,
    version: u8,
    sections: Vec<Section>,
}

impl Station {
    fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            sections: (0..10).map(|_| random()).collect(),
        }
    }

    fn days_left(&self) -> usize {
        self.sections.iter().filter(|m| m.active)
        .count()
    }

    fn working_sections(&self) -> Vec<String> {
        self.sections.iter().filter(|m| m.active)
        .map(|m| m.name.to_string())
        .collect()
    }

    fn broken_sections(&self) -> Vec<String> {
        self.sections.iter().filter(|m| !m.active)
        .map(|m| m.name.to_string())
        .collect()
    }
    fn new_day(&mut self) {
        self.break_something();
    }

    fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-Failure {}", &broken_section.name);
        } else {
            println!("(Sections OK)");
        }
    }

    fn status (&self) {
        dbg!(&self); 
    }
}

#[derive(Debug, RandGen, Eq, PartialEq)]
struct Section {
    name: SectionName,
    active: bool,
}

#[derive(Debug, RandGen, Display, EnumString)]
#[derive(Eq, PartialEq)]
enum SectionName {
    AstroScience, Solar, Antenna,
    RadiationMirrors, Sleeping, NuclearGenerator,
    Galley, Transponder, Tracking
}
