mod entities;
mod repositories;
mod state;
mod utils;

use entities::Transaction;
use state::State;
use utils::print_result;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || !args[1].ends_with(".csv") {
        panic!("You must provide a CSV file.\nRun like this: cargo run -- INPUT_FILE.csv > OUTPUT_FILE.csv");
    }
    let mut state = State::new();
    let mut reader = csv::Reader::from_path(&args[1]).unwrap();
    for rec in reader.records() {
        let transaction_result = rec.unwrap().deserialize::<Transaction>(None);
        if transaction_result.is_err() {
            continue;
        }
        let transaction = transaction_result.unwrap();
        state.process_transaction(transaction);
    }
    print_result(state.get_all_accounts());
}
