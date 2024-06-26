use crate::pawn::Pawn;

pub trait Piece: Send + Sync {

    fn get_possible_moves(&self) -> &Vec<String>;
    fn set_possible_moves(&mut self, moves: Vec<String>);
    fn calculate_possible_moves(&self) -> Vec<String>;
    fn get_symbol(&self) -> &String;
}

pub struct DefaultPiece {
    possible_moves: Vec<String>,
    symbol: String,
}

impl DefaultPiece {
    pub fn new() -> DefaultPiece {
        DefaultPiece {
            possible_moves: Vec::new(),
            symbol: "d".to_string(),
        }
    }
}

impl Piece for DefaultPiece {
    fn get_possible_moves(&self) -> &Vec<String> {
        &self.possible_moves
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {}

    fn calculate_possible_moves(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}

unsafe impl Send for DefaultPiece {}
unsafe impl Sync for DefaultPiece {}