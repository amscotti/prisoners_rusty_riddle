use rand::prelude::*;
use rayon::prelude::*;

const NUMBER_OF_PRISONERS: u32 = 100;

pub struct Session {
    number_of_sessions: u32,
    search: fn(&[u32], u32) -> bool,
}

impl Session {
    pub fn new(number_of_sessions: u32, search: fn(&[u32], u32) -> bool) -> Self {
        Self {
            number_of_sessions,
            search,
        }
    }

    fn session(&self, _s: u32) -> bool {
        let mut rng = rand::thread_rng();
        let mut boxes: Vec<u32> = (0..NUMBER_OF_PRISONERS).collect();
        boxes.shuffle(&mut rng);

        (0..NUMBER_OF_PRISONERS)
            .into_par_iter()
            .all(|p| (self.search)(&boxes, p))
    }

    pub fn run_sessions(&self) -> usize {
        (0..self.number_of_sessions)
            .collect::<Vec<u32>>()
            .par_iter()
            .map(|s| self.session(*s))
            .filter(|s| *s)
            .count()
    }
}
