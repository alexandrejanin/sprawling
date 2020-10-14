use crate::race::RaceDatabase;
use crate::City;
use rand::rngs::StdRng;

pub struct GameState {
    rng: StdRng,
    races: RaceDatabase,
    city: City,
}

impl GameState {
    pub fn new(mut rng: StdRng, races: RaceDatabase) -> Self {
        Self {
            city: City::new(races.random(), &mut rng),
            rng,
            races,
        }
    }

    pub fn update_city(&mut self) {
        self.city = City::new(self.races.random(), &mut self.rng)
    }

    pub fn to_text(&self) -> String {
        format!(
            "You are in {}.\n{}",
            self.city.name(),
            self.city.description()
        )
    }
}
