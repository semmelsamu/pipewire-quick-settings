mod cli;
mod models;
mod pipewire;
mod printers;
mod utils;

use crate::cli::main::cli_loop;

fn main() {
    cli_loop();
}
