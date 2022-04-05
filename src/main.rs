mod args;
use args::Args;
use std::time::Instant;
use std::env::args;

fn main() {
    let startup = Instant::now();
    //let mut board = Board::default();
    let args = Args::parse(args().collect::<Vec<String>>());
    args.handle_board();
    println!("completed in {:#?}", startup.elapsed());
}
