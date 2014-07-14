use std::collections::TreeMap;
use Location::Loc;


enum PieceType {
	Army,
	Fleet
}

struct Piece {
	player   : String,
	pieceType: PieceType
}

pub struct Model {
	locs  : TreeMap<String, Box<Loc>>,
	pieces: TreeMap<String, Vec<Piece>>
}

impl Model {
	pub fn new(filename: &str) -> Model { // TODO return Result
		Model {
			locs  : Loc::from_file(filename),
			pieces: TreeMap::new()
		}
	}
}

