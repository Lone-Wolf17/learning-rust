pub mod station;
pub mod section;

use std::str::FromStr;

use inquire::{Select, Text};
use section::SectionName;
use station::Station;



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

