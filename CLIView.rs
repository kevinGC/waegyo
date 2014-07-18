use std::collections::TreeMap;
use player::PieceType;


#[deriving(Show)]
pub struct View {
	locs: TreeMap<String, (String, PieceType)>
}

impl View {
	pub fn new() -> View {
		return View {
			locs: TreeMap::new()
		}
	}

	pub fn set_piece(&mut self, loc_name: &String, nation: &String, piece_type: PieceType) {
		self.locs.insert(loc_name.clone(), (nation.clone(), piece_type));
	}

	pub fn clear_piece(&mut self, loc_name: &String) {
		self.locs.remove(loc_name);
	}

	pub fn show(&self) {
		for (loc_name, &(ref nation_name, piece_type)) in self.locs.iter() {
			println!("{}: {} {}", loc_name, nation_name, piece_type);
		}
	}
}
