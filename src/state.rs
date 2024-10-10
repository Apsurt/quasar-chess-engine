use core::fmt;
use std::usize;

use crate::{moves::Move, pieces::{name_to_type, symbol_to_name, Piece, PieceColor, PieceType}};
use glam::IVec2 as Vec2;

#[derive(Debug)]
pub struct State {
    pieces: Vec<Piece>,
    pub to_move: PieceColor,
    pub half_moves: usize,
    pub full_moves: usize,
    pub promotion_ranks: Vec<i32>
}

impl State {
    pub fn from_fen(fen: String) -> State {
        let mut pieces = vec![];
        let mut x: i32 = 1;
        let mut y: i32 = 8;
        
        for symbol in fen.chars() {
            if symbol == ' ' {
                break;
            }
            if symbol == '/' {
                y -= 1;
                x = 1;
                continue;
            }
            if symbol.is_digit(10) {
                x += symbol as i32;
                continue;
            }
            let piece_color;
            if symbol.is_lowercase() {
                piece_color = PieceColor::BLACK;
            }
            else {
                piece_color = PieceColor::WHITE;
            }
            let piece_type: PieceType = name_to_type(symbol_to_name(symbol));
            pieces.push(Piece::new(piece_color, piece_type, Vec2::new(x, y)));
            x += 1
        }
        
        let to_move = PieceColor::WHITE;
        let half_moves = 0;
        let full_moves = 0;
        let promotion_ranks = vec![1,8];
        
        State { pieces, to_move, half_moves, full_moves, promotion_ranks }
    }
    
    pub fn get_piece_at(&self, pos: Vec2) -> Option<&Piece> {
        for piece_idx in 0..self.pieces.len() {
            let piece_pos = self.pieces[piece_idx].get_position();
            if (piece_pos.x == pos.x) && (piece_pos.y == pos.y) {
                return Some(&self.pieces[piece_idx])
            }
        }
        return None
    }
    
    fn find_piece_idx(&self, piece: Piece) -> Option<usize> {
        let mut same_idx: usize = usize::MAX;
        for idx in 0..self.pieces.len() {
            if self.pieces[idx] == piece {
                same_idx = idx;
                break;
            }
        }
        if same_idx < self.pieces.len() {
            return Some(same_idx);
        }
        None
    }
    
    fn switch_to_move(&self) -> PieceColor {
        match self.to_move {
            PieceColor::BLACK => PieceColor::WHITE,
            PieceColor::WHITE => PieceColor::BLACK,
        }
    }
    
    pub fn make_move(self, mov: Move) -> State {
        let pieces = self.pieces.clone();
        let to_move: PieceColor = self.switch_to_move();
        let half_moves: usize = self.half_moves + 1;
        let full_moves: usize = self.full_moves + match to_move {
            PieceColor::WHITE => 1,
            PieceColor::BLACK => 0,
        };
        let promotion_ranks = self.promotion_ranks;
        
        let mut state = State { pieces, to_move, half_moves, full_moves, promotion_ranks };
        
        //todo edge cases (en passant, castling etc)
        let idx = state.find_piece_idx(mov.piece).expect("Piece does not exist.");
        state.pieces[idx].set_position(mov.end);
        
        let idx = state.find_piece_idx(mov.target.unwrap()).expect("Target doesn't exist");
        state.pieces[idx].capture();

        state
    }
    
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut result = String::new();
            for y in (1..9).rev() {
                for x in 1..9 {
                    let piece = self.get_piece_at(Vec2::new(x, y));
                    let piece: &Piece = match piece {
                        Some(_) => piece.unwrap(),
                        None => {
                            result += ".";
                            continue
                        }
                    };
                    result += piece.get_symbol().to_string().as_str();
                }
                result += "\n";
            }
            write!(f, "{}", result,)
       }
}