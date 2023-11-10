mod cli;
mod record;
mod store;

use crate::cli::Cli;
use crate::cli::Commands;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    // let mut store = Box::new(store::LocalStore::new()) as Box<dyn store::Store>;
    // file store
    let mut store =
        Box::new(store::FileStore::new("store.json".to_string())) as Box<dyn store::Store>;
    let mut command = record::Command::new(&args.command, &mut store);
    println!("{}", command.execute());
}
