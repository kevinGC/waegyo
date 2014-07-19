use std::rc::{Rc, Weak};
use std::cell::RefCell;
use player::Player;

#[deriving(Show)]
pub enum PieceType {
	Army,
	Fleet
}

pub struct Piece {
	piece_t: PieceType,
	player : Weak<RefCell<Player>>
}

impl Piece {
	pub fn new(piece_t: PieceType, player: Rc<RefCell<Player>>) -> Piece {
		return Piece {
			piece_t: piece_t,
			player : player.downgrade()
		}
	}
}
