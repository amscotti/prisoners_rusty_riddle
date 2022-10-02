use clap::{Parser, ValueEnum};
use session::Session;

mod session;

const NUMBER_OF_SESSIONS: u32 = 1000000;

#[derive(ValueEnum, Clone, PartialEq, Eq, Debug)]
pub enum Strategy {
    Loop,
    Random,
}

/// 100 Prisoners Riddle Simulator
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Number of sessions to run
    #[clap(short, long, value_parser, default_value_t = NUMBER_OF_SESSIONS)]
    number_of_sessions: u32,

    /// Search strategy to use by the prisoners
    #[clap(value_enum, default_value_t = Strategy::Loop)]
    strategy: Strategy,
}

fn loop_search(boxes: &[u32], prisoner: u32) -> bool {
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

fn random_search(boxes: &[u32], prisoner: u32) -> bool {
    let guesses: Vec<usize> = (0..(boxes.len() / 2)).collect();

    for i in guesses {
        let value = boxes[i as usize];
        if value == prisoner {
            return true;
        }
    }

    false
}

fn main() {
    let args = Args::parse();

    let strategy = match args.strategy {
        Strategy::Loop => loop_search,
        Strategy::Random => random_search,
    };

    let count = Session::new(args.number_of_sessions, strategy).run_sessions();

    println!(
        "Count of success {} - Percentage {:.2}% using {:?} strategy",
        count,
        (count as f64 / args.number_of_sessions as f64) * 100.0,
        args.strategy
    )
}
