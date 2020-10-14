use crate::race::Race;
use rand::Rng;

pub struct City {
    race: Race,
    name: String,
    population: u32,
}

impl City {
    pub fn new<R: Rng>(race: Race, rng: &mut R) -> Self {
        Self {
            name: race.location_name(rng),
            race,
            population: rng.gen_range(0, 100) * rng.gen_range(0, 100),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn population(&self) -> &u32 {
        &self.population
    }

    pub fn description(&self) -> String {
        format!(
            "{} is a {} {} with a population of {}.",
            self.name(),
            self.race.adjective(),
            match self.population {
                0 => "ghost town",
                1..=1000 => "village",
                1001..=5000 => "town",
                _ => "city",
            },
            self.population,
        )
    }
}
