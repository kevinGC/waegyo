use std::collections::TreeMap;
use std::io::File;
use std::str::*;
use std::rc::Rc;
use std::cell::RefCell;
use serialize::json;
use Location::Loc;
use player::Player;
use CLIView::View;


// TODO when to use Rc vs Arc?
// TODO Cell vs RefCell
// TODO I want this RefCell junk gone. Possible?

pub type LocsType = TreeMap<String, Rc<RefCell<Loc>>>;
pub type PlayersType = Vec<Rc<RefCell<Player>>>;

pub struct Model {
	locs   : LocsType,
	// locs   : LocsType,
	players: PlayersType,
	views  : Vec<Rc<View>>
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
			locs   : locs,
			players: players,
			views  : Vec::new()
		}
	}

	pub fn add_view(&mut self, view: Rc<View>) {
		self.views.push(view);
	}

	pub fn remove_view(&self, view: &View) {
		// TODO
	}
}
