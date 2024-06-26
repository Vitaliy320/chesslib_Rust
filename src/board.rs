use std::collections::HashMap;
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

use crate::square::Square;
use crate::position::Position;

pub struct Board {
    number_of_columns: u32,
    number_of_rows: u32,
    board_fen: String,
    columns: String,
    rows: String,
    _position: Arc<RwLock<Position>>,
    _board_rows: Vec<Vec<Arc<RwLock<Square>>>>,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let cloned_position = self._position.clone();
        let cloned_board_rows: Vec<Vec<Arc<RwLock<Square>>>> = self._board_rows.iter()
            .map(|row| row.iter().map(|cell| Arc::clone(cell)).collect()).collect();


        let mut board = Board {
            columns: self.columns.clone(),
            number_of_columns: self.number_of_columns,
            rows: self.rows.clone(),
            number_of_rows: self.number_of_rows,
            board_fen: self.board_fen.clone(),
            _position: cloned_position,
            _board_rows: cloned_board_rows,
        };
        board.update_board(board.board_fen.clone());
        board
    }
}

impl Board {
    pub fn new(columns: String, number_of_columns: u32, rows: String, number_of_rows: u32,
               fen: String) -> Board {
        let position = Arc::new(RwLock::new(Position::new(
            columns.clone(),
            number_of_columns,
            rows.clone(),
            number_of_rows,
            fen.clone()
        )));

        let mut board = Board {
            columns,
            number_of_columns,
            rows,
            number_of_rows,
            board_fen: fen,
            _position: Arc::clone(&position),
            _board_rows: Vec::new(),
        };
        board.initialize_board();
        let cloned_board = board.clone();
        board.update_board(board.clone().board_fen);
        board
    }

    fn initialize_board(&mut self) {
        let mut coordinates_string: String;
        let mut column: char;
        let mut row: char;
        let mut squares: HashMap<String, Arc<RwLock<Square>>> = HashMap::new();

        for i in 0..self.number_of_columns {
            column = self.columns.chars().nth(i as usize).unwrap_or('\0');
            for j in 0..self.number_of_rows {
                row = self.rows.chars().nth(j as usize).unwrap_or('\0');

                coordinates_string = column.to_string() + &row.to_string();
                squares.insert(coordinates_string, Arc::new(RwLock::new(Square::new((column, row)))));
                // println!("Coordinates: {:?}", squares.get("a1").unwrap().borrow_mut().get_coordinates())
            }
        }

        if let Ok(mut guard) = self._position.write(){
            let mut pos = guard.deref_mut();
            pos.set_squares(squares);
        }
        // self._position.borrow_mut().set_squares(squares);
    }

    pub fn get_position(&self) -> Arc<RwLock<Position>> {
        self._position.clone()
    }

    pub fn update_board(&mut self, fen: String) {
        if let Ok(mut guard) = self._position.write() {
            let mut pos = guard.deref_mut();
            pos.position_from_fen(fen);
        }
        // self._position.borrow_mut().position_from_fen(fen);
    }

    pub fn make_move(&mut self, move_from: (char, char), move_to: (char, char)) {
        if let Ok(mut guard) = self._position.write() {
            let mut pos = guard.deref_mut();
            pos.make_move(move_from, move_to);
        }
        // self._position.borrow().make_move(move_from, move_to);
    }

    pub fn make_move_str(&mut self, move_from: String, move_to: String) {
        if let Ok(mut guard) = self._position.write() {
            let mut pos = guard.deref_mut();
            pos.make_move(
                (move_from.chars().nth(0).unwrap(), move_from.chars().nth(1).unwrap()),
                (move_to.chars().nth(0).unwrap(), move_to.chars().nth(1).unwrap())
            );
            let s = pos.get_square_by_coordinates(('e', '2'))
                .read()
                .unwrap()
                .square_to_str();
            println!("make_move_str square: {}", s);
        }
        // self._position.borrow().make_move(
        //     (move_from.chars().nth(0).unwrap(), move_from.chars().nth(1).unwrap()),
        //     (move_to.chars().nth(0).unwrap(), move_to.chars().nth(1).unwrap())
        // );
    }

    pub fn create_board_rows(&mut self) {
        let mut current_row: Vec<Arc<RwLock<Square>>>;
        let mut all_rows: Vec<Vec<Arc<RwLock<Square>>>> = Vec::new();

        for j in (0..self.number_of_rows).rev() {
            current_row = Vec::new();
            if let Ok(mut guard) = self._position.write() {
                let mut pos = guard.deref_mut();
                for i in 0..self.number_of_columns {
                    current_row.push(
                        pos.get_square_by_coordinates(
                            (self.columns.chars().nth(i as usize).unwrap(),
                            self.rows.chars().nth(j as usize).unwrap()))

                    );
                }
            }
            all_rows.push(current_row);
        }

        self._board_rows = all_rows;
    }

    pub fn board_to_string(&mut self) -> String {
        self.create_board_rows();

        let mut rows_vector: Vec<String> = Vec::new();

        let mut row_str: String = "".to_string();
        let mut rows_str = "                  \n                  ".to_string();
        rows_vector.push("                  ".to_string());
        rows_vector.push("                  ".to_string());

        let mut row: &Vec<Arc<RwLock<Square>>> = &Vec::new();

        for i in 0..self._board_rows.len() {
            // row_str = "".to_string();
            let mut current_row: String = "".to_string();
            let val = (8 - i).to_string() + &' '.to_string();
            current_row.push_str(val.as_str());
            // row_str += &val;

            row = &self._board_rows[i];
            for mut square in row {
                if let Ok(mut guard) = square.write() {
                    let sq = guard.deref_mut();
                    match sq.get_piece() {
                        Some(piece) => {
                            current_row += piece.get_symbol();
                            current_row += " ";
                            row_str += piece.get_symbol();
                            row_str += " ";
                        },
                        None => {
                            current_row.push_str("  ");
                            row_str += " ";
                        },
                    }
                }
            }
            rows_str.push_str("\n");
            rows_str.push_str(current_row.as_str());
            // rows_str = "\n".to_owned() + row_str.as_str();
            rows_vector.push(current_row);
            // rows_str += row_str.as_str();
        }

        rows_vector.push("  A B C D E F G H ".to_string());
        rows_str += "\n  A B C D E F G H ";
        rows_str
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
