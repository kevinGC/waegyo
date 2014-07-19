use std::collections::TreeMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use serialize::json;
use WorldModel::LocsType;
use piece::{PieceType, Piece};
use player::Player;


static comment: &'static str = "comment";

enum Terrain {
	Land,
	Sea
}

pub struct Loc {
	// adjacent     : Vec<String>,
	adjacent     : RefCell<Vec<Weak<RefCell<Loc>>>>,
	terrain      : Terrain,
	piece        : RefCell<Option<Piece>>, // TODO can I avoid this RefCell?
	supply_center: bool,
	display_name : String
}

impl Loc {
	pub fn from_json_obj(json_obj: &json::Object) -> LocsType {
		let parent_obj = json_obj.find(&("locations").to_string()).unwrap().as_object().unwrap();
		let mut locs: LocsType = TreeMap::new();

		// insert blank locations
		for (key, sub_json_obj) in parent_obj.iter() {
			if key.as_slice().contains(comment)  { // skip comments
				continue;
			}
			let obj = sub_json_obj.as_object().unwrap();

			let mut loc = Loc::new();
			// configure properties
			let terrain_key = &("terrain").to_string();
			let terrain_str = obj.find(terrain_key).unwrap().as_string().unwrap();
			loc.terrain =
				if terrain_str.to_string() == ("Land").to_string() {
					Land
				} else {
					Sea
				};
			let disp_key = &("display_name").to_string(); // TODO make these Strings static
			let disp_str = obj.find(disp_key).unwrap().as_string().unwrap();
			loc.display_name = disp_str.to_string();

			locs.insert(key.clone(), Rc::new(RefCell::new(loc)));
		}

		// configure each location
		for (key, sub_json_obj) in parent_obj.iter() {
			if key.as_slice().contains(comment)  { // skip comments
				continue;
			}
			let obj = sub_json_obj.as_object().unwrap();

			let loc = locs.find(key).unwrap();
			// add adjacent locations
			let adj_key = &("adjacent").to_string();
			let mut it = obj.find(adj_key).unwrap().as_list().unwrap().iter();
			for adjacent in it {
				let adj_string = adjacent.as_string().unwrap().to_string();
				let adj_loc = locs.find(&adj_string).unwrap();
				loc.borrow_mut().adjacent.borrow_mut().push(adj_loc.downgrade());
			}
		}
		locs
	}

	pub fn set_piece(&mut self, player: Rc<RefCell<Player>>, piece_t: PieceType) {
		self.piece = RefCell::new(Some(Piece::new(piece_t, player)));
	}

	pub fn remove_piece(&mut self) {
		self.piece = RefCell::new(None);
	}

	fn new() -> Loc {
		Loc {
			adjacent     : RefCell::new(Vec::new()),
			terrain      : Land,
			display_name : String::new(),
			piece        : RefCell::new(None),
			supply_center: false
		}
	}

}
