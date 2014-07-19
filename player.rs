use std::collections::TreeMap;
use std::rc::Rc;
use std::cell::RefCell;
use serialize::json;
use WorldModel::{LocsType, PlayersType};
use Location::Loc;
use piece::{Fleet, Army};


pub struct Player {
	display_name: String,
	// pieces      : TreeMap<String, PieceType>
	piece_locs  : RefCell<Vec<Rc<RefCell<Loc>>>>
}

// TODO find_equiv and hash map?
// TODO naming with player vs nation
impl Player {
	// TODO some of this unwrapping has to be handled for meaninful feedback
	// regarding ill-formed input files
	pub fn from_json_obj(parent_obj: &json::Object, locs: &mut LocsType) -> PlayersType {
		let mut players = Vec::new();

		let nations: &TreeMap<String, json::Json> = parent_obj.find(&("nations").to_string()).unwrap()
			.as_object().unwrap();
		for (nation_name, nation_json) in nations.iter() {
			println!("Processing nation: {}", nation_name);
			let nation = nation_json.as_object().unwrap();
			let display_name = nation.find(&("display_name").to_string()).unwrap()
				.as_string().unwrap();

			let mut player = Rc::new(RefCell::new(Player {
				display_name: display_name.to_string(),
				piece_locs  : RefCell::new(Vec::new())
			}));

			let pieces = nation.find(&("pieces").to_string()).unwrap().as_list()
				.unwrap();
			for piece_json in pieces.iter() {
				let piece = piece_json.as_object().unwrap();
				let piece_loc = piece.find(&("loc").to_string()).unwrap().as_string()
					.unwrap();
				let piece_loc_string = piece_loc.to_string();
				let piece_type_string = piece.find(&("type").to_string()).unwrap()
					.as_string().unwrap();
				let piece_type = if piece_type_string == "fleet" {
						Fleet
					} else {
						Army
					};

				// add piece to location, piece to player, and player to players
				let loc: &Rc<RefCell<Loc>> = locs.find(&piece_loc_string).unwrap();
				loc.borrow_mut().set_piece(player.clone(), piece_type);
				player.borrow_mut().piece_locs.borrow_mut().push(loc.clone());
			}
			players.push(player);
		}

		// println!("players: {}", players);
		players
	}
}
