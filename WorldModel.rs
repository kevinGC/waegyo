use std::collections::TreeMap;
use std::io::File;
use std::str::*;
use serialize::json;
use Location::Loc;
use player::Player;

pub type LocsType = TreeMap<String, Loc>;
// pub type PlayersType = TreeMap<String, Vec<Piece>>;

#[deriving(Show)]
pub struct Model {
	locs   : LocsType,
	players: Vec<Player>
}

impl Model {
	pub fn new(filename: &str) -> Model { // TODO return Result
		let file_contents = File::open(&Path::new(filename)).read_to_end();
		let contents      = from_utf8_owned(file_contents.unwrap()).unwrap();
		let parent        = json::from_str(contents.as_slice()).unwrap();
		let parent_obj    = parent.as_object().unwrap();

		let mut locs = Loc::from_json_obj(parent_obj);
		let players = Player::from_json_obj(parent_obj, &mut locs);
		Model {
			locs  : locs,
			players: players
		}
	}
}

