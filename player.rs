use std::collections::TreeMap;
use serialize::json;
use WorldModel::LocsType;

#[deriving(Show)]
enum PieceType {
	Army,
	Fleet
}

#[deriving(Show)]
pub struct Player {
	display_name: String,
	pieces: TreeMap<String, PieceType>
}

impl Player {
	pub fn from_json_obj(parent_obj: &json::Object, locs: &mut LocsType) -> Vec<Player> {
		let players = Vec::new();

		let nations: &TreeMap<String, json::Json> = parent_obj.find(&String::from_str("nations")).unwrap()
			.as_object().unwrap();
		for (nation_name, nation_json) in nations.iter() {
			let nation = nation_json.as_object().unwrap();
			let display_name = nation.find(&String::from_str("display_name")).unwrap()
				.as_string().unwrap();

			let mut player = Player {
				display_name: String::from_str(display_name),
				pieces      : TreeMap::new()
			};

			let pieces = nation.find(&String::from_str("pieces")).unwrap().as_list()
				.unwrap();
			for piece_json in pieces.iter() {
				let piece = piece_json.as_object().unwrap();
				let piece_loc = piece.find(&String::from_str("loc")).unwrap().as_string()
					.unwrap();
				let piece_loc_string = String::from_str(piece_loc);
				let piece_type_string = piece.find(&String::from_str("type")).unwrap()
					.as_string().unwrap();
				let piece_type = if piece_type_string == "fleet" {
						Fleet
					} else {
						Army
					};

				// add piece to location
				locs.find_mut(&piece_loc_string).unwrap().set_piece(nation_name.clone());
				// add piece to player
				player.pieces.insert(piece_loc_string, piece_type);
			}
		}

		players
	}
}
