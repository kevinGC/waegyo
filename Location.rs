use std::collections::TreeMap;
use serialize::json;
use WorldModel::LocsType;


static comment: &'static str = "comment";

#[deriving(Show)]
enum Terrain {
	Land,
	Sea
}

#[deriving(Show)]
pub struct Loc {
	adjacent     : Vec<String>,
	terrain      : Terrain,
	piece        : Option<String>,
	supply_center: bool,
	display_name : String
}

impl Loc {
	pub fn from_json_obj(json_obj: &json::Object) -> LocsType {
		let parent_obj = json_obj.find(&("locations").to_string()).unwrap().as_object().unwrap();
		let mut locs: LocsType = TreeMap::new();

		// insert blank locations
		for (key, _) in parent_obj.iter() {
			if key.as_slice().contains(comment)  { // skip comments
				continue;
			}

			locs.insert(key.clone(), Loc::new());
		}

		// configure each location
		for (key, sub_json_obj) in parent_obj.iter() {
			if key.as_slice().contains(comment)  { // skip comments
				continue;
			}

			let obj = sub_json_obj.as_object().unwrap();
			let loc = locs.find_mut(key).unwrap();

			// configure properties
			let terrain_key    = &("terrain").to_string();
			let terrain_str    = obj.find(terrain_key).unwrap().as_string().unwrap();
			loc.terrain =
				if terrain_str.to_string() == ("Land").to_string() {
					Land
				} else {
					Sea
				};
			let disp_key = &("display_name").to_string(); // TODO make these Strings static
			let disp_str = obj.find(disp_key).unwrap().as_string().unwrap();
			loc.display_name = disp_str.to_string();

			// add adjacent locations
			let adj_key = &("adjacent").to_string();
			let mut it = obj.find(adj_key).unwrap().as_list().unwrap().iter();
			for adjacent in it {
				let adj_string = adjacent.as_string().unwrap().to_string();
				loc.adjacent.push(adj_string);
			}
		}
		locs
	}

	pub fn set_piece(&self, player: String) {
		self.piece == Some(player);
	}

	fn new() -> Loc {
		Loc {
			adjacent     : Vec::new(),
			terrain      : Land,
			display_name : String::new(),
			piece        : None,
			supply_center: false
		}
	}

}
