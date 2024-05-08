use crate::piece::Piece;

pub struct Rook {
    color: char,
    possible_moves: Vec<String>,
    name: String,
    symbol: String,
}

impl Rook {
    pub fn new(color: char) -> Rook {
        let symbol = if color.to_lowercase().next() == Some('w') {
            "R".to_string()
        } else {
            "r".to_string()
        };
        Rook {color: color,
            possible_moves: Vec::new(),
            name: String::from("Rook"),
            symbol: symbol
        }
    }
}

impl Piece for Rook {
    fn get_possible_moves(&self) -> &Vec<String> {
        &self.possible_moves
    }

    fn set_possible_moves(&mut self, moves: Vec<String>) {
        self.possible_moves = moves
    }

    fn calculate_possible_moves(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_symbol(&self) -> &String {
        &self.symbol
    }
}