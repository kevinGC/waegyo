#![feature(globs)]

// TODO why can't this go in Location?
extern crate serialize;

use CLIController::run;

mod WorldModel;
mod Location; // TODO why is this here and not in WorldModel?
mod player;
mod piece;
mod CLIController;
mod CLIView;

// TODO need to support multiple coasts...
// TODO so it turns out there are weak references in Rust. Have to redo a lot...


fn main() {
	println!("Hello, 외교");
	CLIController::run(); // TODO platform-specific controllers
}
