use clap::Parser;
use rand::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

const NUMBER_OF_PRISONERS: u32 = 100;
const NUMBER_OF_SESSIONS: u32 = 1000000;

/// 100 Prisoners Riddle Simulator
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Number of sessions to run
    #[clap(short, long, value_parser, default_value_t = NUMBER_OF_SESSIONS)]
    number_of_sessions: u32,
}

fn search(boxes: &Vec<u32>, prisoner: u32) -> bool {
    let mut index = prisoner;
    let guesses = boxes.len() / 2;

    for _i in 0..guesses {
        let value = boxes[index as usize];
        if value == prisoner {
            return true;
        }
        index = value
    }

    false
}

#[cfg(not(target_arch = "wasm32"))]
fn session(_s: u32) -> bool {
    let mut rng = rand::thread_rng();
    let mut boxes: Vec<u32> = (0..NUMBER_OF_PRISONERS).collect();
    boxes.shuffle(&mut rng);

    (0..NUMBER_OF_PRISONERS)
        .into_par_iter()
        .all(|p| search(&boxes, p))
}

#[cfg(target_arch = "wasm32")]
fn session(_s: u32) -> bool {
    let mut rng = rand::thread_rng();
    let mut boxes: Vec<u32> = (0..NUMBER_OF_PRISONERS).collect();
    boxes.shuffle(&mut rng);

    (0..NUMBER_OF_PRISONERS).all(|p| search(&boxes, p))
}

#[cfg(not(target_arch = "wasm32"))]
fn run_sessions(number_of_sessions: u32) -> usize {
    (0..number_of_sessions)
        .into_par_iter()
        .map(session)
        .filter(|s| *s)
        .count()
}

#[cfg(target_arch = "wasm32")]
fn run_sessions(number_of_sessions: u32) -> usize {
    (0..number_of_sessions).map(session).filter(|s| *s).count()
}

fn main() {
    let args = Args::parse();

    let count = run_sessions(args.number_of_sessions);

    println!(
        "Count of success {} - Percentage {:.2}%",
        count,
        (count as f64 / args.number_of_sessions as f64) * 100.0
    )
}
