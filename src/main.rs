use std::sync::mpsc;

mod utils;

fn main() {
    let (tx, rx) = mpsc::channel();
    utils::commandline(tx);
    utils::init(rx);
}
