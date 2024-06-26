use std::sync::Arc;

use crate::piece::Piece;

pub struct Square {
    _coordinates: (char, char),
    _piece: Option<Arc<dyn Piece + Sync + Send>>,
}

impl Square {
    pub fn new(coordinates: (char, char)) -> Square {
        Square { _coordinates: coordinates, _piece: None }
    }

    pub fn get_piece(&self) -> Option<Arc<dyn Piece + Sync + Send>> {
        self._piece.clone()
    }

    pub fn set_piece(&mut self, piece: Arc<dyn Piece + Sync + Send>) {
        self._piece = Some(piece);
    }

    pub fn remove_piece(&mut self) {
        self._piece = None;
    }

    pub fn capture_piece(&mut self, capturing_piece: Arc<dyn Piece + Sync + Send>) {
        self._piece = Some(capturing_piece);
    }

    pub fn get_coordinates(&self) -> (char, char) {
        self._coordinates
    }

    pub fn square_to_str(&self) -> String {
        let piece = self.get_piece();
        let coordinates = self.get_coordinates().0.to_string() + &self.get_coordinates().1.to_string();
        match piece {
            Some(p) => coordinates + p.get_symbol(),
            None => coordinates.to_string(),
        }
    }
}