#![feature(globs)]
extern crate serialize;

use WorldModel::Model;

mod WorldModel;
mod Location;


fn main() {
	println!("Hello, 외교");
	let model = Model::new("defaultWorld.json");
}
