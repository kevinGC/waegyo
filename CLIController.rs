#![feature(globs)]

extern crate serialize; // TODO why can't this go in Location?

use WorldModel::Model;

mod WorldModel;
mod Location; // TODO why is this here and not in WorldModel?
mod player;
// mod util;


fn main() {
	println!("Hello, 외교");
	let model = Model::new("defaultWorld.json");
	println!("{}", model);
}
