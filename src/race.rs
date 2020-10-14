use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Race {
    single_name: String,
    collective_name: String,
    adjective: String,
    adjectives: Vec<String>,
    locations: Vec<String>,
}

impl Race {
    pub fn single_name(&self) -> &str {
        &self.single_name
    }
    pub fn collective_name(&self) -> &str {
        &self.collective_name
    }
    pub fn adjective(&self) -> &str {
        &self.adjective
    }
    pub fn adjectives(&self) -> &[String] {
        &self.adjectives
    }
    pub fn locations(&self) -> &[String] {
        &self.locations
    }
    pub fn location_name<R: Rng>(&self, rng: &mut R) -> String {
        Self::from_list(rng, self.adjectives())
            + &Self::from_list(rng, self.locations()).to_lowercase()
    }

    fn from_list<R: Rng, T: Clone>(rng: &mut R, list: &[T]) -> T {
        list[rng.gen_range(0, list.len())].clone()
    }
}

pub struct RaceDatabase {
    races: Vec<Race>,
}

impl RaceDatabase {
    pub fn random(&self) -> Race {
        self.races[thread_rng().gen_range(0, self.races.len())].clone()
    }
}

impl Default for RaceDatabase {
    fn default() -> Self {
        Self {
            races: vec![
                Race {
                    single_name: "Human".to_string(),
                    collective_name: "Humans".to_string(),
                    adjective: "human".to_string(),
                    adjectives: vec![
                        "Light".to_string(),
                        "Dark".to_string(),
                        "High".to_string(),
                        "Sun".to_string(),
                        "Frost".to_string(),
                        "Dead".to_string(),
                    ],
                    locations: vec![
                        "Well".to_string(),
                        "Spring".to_string(),
                        "Haven".to_string(),
                        "Hill".to_string(),
                        "Field".to_string(),
                        "Stone".to_string(),
                        "Wood".to_string(),
                        "Helm".to_string(),
                    ],
                },
                Race {
                    single_name: "Orc".to_string(),
                    collective_name: "Orcs".to_string(),
                    adjective: "orc".to_string(),
                    adjectives: vec![
                        "Ak".to_string(),
                        "Lok".to_string(),
                        "Tar".to_string(),
                        "Ag".to_string(),
                    ],
                    locations: vec![
                        "Torn".to_string(),
                        "Krek".to_string(),
                        "Tark".to_string(),
                        "Gûl".to_string(),
                    ],
                },
            ],
        }
    }
}
