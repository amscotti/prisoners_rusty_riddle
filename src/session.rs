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

    fn session(&self) -> bool {
        let mut rng = rand::thread_rng();
        let mut boxes: Vec<u32> = (0..NUMBER_OF_PRISONERS).collect();
        boxes.shuffle(&mut rng);

        (0..NUMBER_OF_PRISONERS)
            .into_par_iter()
            .all(|p| (self.search)(&boxes, p))
    }

    pub fn run_sessions(&self) -> usize {
        (0..self.number_of_sessions)
            .into_par_iter()
            .map(|_| self.session())
            .filter(|s| *s)
            .count()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Strategy;
    use assert_approx_eq::assert_approx_eq;

    const TEST_SESSIONS: u32 = 1000;

    fn assert_success_rate(
        number_of_sessions: u32,
        search: fn(&[u32], u32) -> bool,
        expected_success_rate: f64,
    ) {
        let count = Session::new(number_of_sessions, search).run_sessions();
        let success_rate = (count as f64) / (number_of_sessions as f64);
        assert_approx_eq!(success_rate, expected_success_rate, 0.05);
    }

    #[test]
    fn test_loop_search_success_rate() {
        assert_success_rate(TEST_SESSIONS, crate::loop_search, 0.31);
    }

    #[test]
    fn test_random_search_success_rate() {
        assert_success_rate(TEST_SESSIONS, crate::random_search, 0.0);
    }

    #[test]
    fn test_loop_search_strategy() {
        let args = crate::Args {
            number_of_sessions: TEST_SESSIONS,
            strategy: Strategy::Loop,
        };
        let strategy = match args.strategy {
            Strategy::Loop => crate::loop_search,
            _ => panic!("Unexpected strategy"),
        };
        assert_success_rate(TEST_SESSIONS, strategy, 0.31);
    }

    #[test]
    fn test_random_search_strategy() {
        let args = crate::Args {
            number_of_sessions: TEST_SESSIONS,
            strategy: Strategy::Random,
        };
        let strategy = match args.strategy {
            Strategy::Random => crate::random_search,
            _ => panic!("Unexpected strategy"),
        };
        assert_success_rate(TEST_SESSIONS, strategy, 0.0);
    }
}