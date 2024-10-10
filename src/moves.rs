use core::fmt;

use glam::IVec2 as Vec2;
use crate::pieces::{Piece, PieceColor, PieceType};
use crate::state::State;

#[derive(Debug, Clone)]
pub struct Move {
    pub start: Vec2,
    pub end: Vec2,
    pub piece: Piece,
    pub target: Option<Piece>,
    pub castling: bool,
    pub castling_target: Option<Piece>,
    pub en_passant: bool,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(start: Vec2, end: Vec2, piece: Piece, target: Option<Piece>,  promotion: Option<PieceType>, castling: bool, castling_target: Option<Piece>, en_passant: bool) -> Move {
        Move { start, end, piece, target, castling, castling_target, en_passant, promotion }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut symbol = self.piece.get_symbol();
        match self.piece.get_color() {
            PieceColor::WHITE => {
                symbol = symbol.to_ascii_uppercase()
            },
            PieceColor::BLACK => {}
        };
        write!(f,
            "{}{},{}>{},{}",
            symbol,
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
        )
    }
}

pub struct Generator {
    pub n: Vec<usize>,
    pub buffer: Vec<Move>,
    piece: Piece,
    state: State,
    offsets: Option<Vec<Vec2>>,
}

impl Generator {
    pub fn new(piece: Piece, state: State) -> Generator {
        let n: usize = match piece.get_piece_type() {
            PieceType::PAWN => {3} //move forward, left att, right att
            PieceType::KNIGHT => {8}
            PieceType::BISHOP => {4}
            PieceType::ROOK => {4}
            PieceType::QUEEN => {8}
            PieceType::KING => {10} //8 directions and 2 castling directions
            _ => {0}
        };
        
        let n = vec![0; n];
        
        let straight: Vec<Vec2> = vec![Vec2::new( 1,  0), Vec2::new(-1,  0), Vec2::new( 0, -1), Vec2::new( 0,  1)];
        let diagonal: Vec<Vec2> = vec![Vec2::new( 1,  1), Vec2::new( 1, -1), Vec2::new(-1,  1), Vec2::new(-1, -1),];
        let combined: Vec<Vec2> = [straight.clone(), diagonal.clone()].concat();
        
        let offsets: Option<Vec<Vec2>> = match piece.get_piece_type() {
            PieceType::PAWN => {None} ,
            PieceType::KNIGHT => {None},
            PieceType::BISHOP => {Some(diagonal.clone())},
            PieceType::ROOK => {Some(straight.clone())},
            PieceType::QUEEN => {Some(combined.clone())},
            PieceType::KING => {None},
            _ => {None},
        };
        let buffer = vec![];
        Generator { n, buffer, piece, state, offsets }
    }
    
    pub fn is_depleated(&self) -> bool {
        let min_n = self.n.iter().min().unwrap().clone();
        if min_n == usize::MAX {
            true
        }
        else {
            false
        }
    }
    
    fn is_in_bounds(&self, point: Vec2) -> bool {
        let top_left = self.state.config.boundaries[0];
        let bottom_right = self.state.config.boundaries[1];
        if (point.x < bottom_right.x) && (point.x > top_left.x) {
            if (point.y > bottom_right.y) && (point.y < top_left.y) {
                return true;
            }
        }
        false
    }
    
    fn next_pawn_offset(&mut self) -> Option<Move> {
        if self.buffer.len() > 0 {
            return self.buffer.pop();
        }
        //move forward, left att, right att
        let offsets = match self.piece.get_color() {
            PieceColor::WHITE => vec![Vec2::new(0, 1), Vec2::new(-1, 1), Vec2::new(1, 1)],
            PieceColor::BLACK => vec![Vec2::new(0, -1), Vec2::new(1, -1), Vec2::new(-1, -1)]
        };
        let promotions: Vec<Option<PieceType>> = 
            vec![
                Some(PieceType::BISHOP),
                Some(PieceType::KNIGHT),
                Some(PieceType::ROOK),
                Some(PieceType::QUEEN),
                None,
            ];
        
        for idx in 0..self.n.len() {
            if self.n[idx] != 0 {
                continue;
            }
            
            let mul_iter = match idx {
                0 => 1..3,
                _ => 1..2
            };
            
            for mul in mul_iter {
                let start = self.piece.get_position().clone();
                let end = self.piece.get_position().clone() + (offsets[idx] * mul);
                if !self.is_in_bounds(end) {
                    self.n[idx] = usize::MAX;
                    continue;
                }
                for promotion in promotions.iter() {
                    for en_passant in [true, false].iter() {
                        self.buffer.push(
                            Move::new(start, end, self.piece.clone(), None, promotion.clone(), false, None, en_passant.to_owned())
                        );
                    }
                }
            }
            
            self.n[idx] = usize::MAX;
            break;
        }
        
        self.buffer.pop()
    }
    
    fn next_knight_offset(&mut self) -> Option<Move> {
        let offsets: Vec<Vec2> =
            vec![
                Vec2::new(-1,  2),
                Vec2::new( 1,  2),
                Vec2::new(-1, -2),
                Vec2::new( 1, -2),
                Vec2::new( 2, -1),
                Vec2::new( 2,  1),
                Vec2::new(-2, -1),
                Vec2::new(-2,  1),
            ];
        for idx in 0..self.n.len() {
            if self.n[idx] != 0 {
                self.n[idx] = usize::MAX;
                continue;
            }
            let start = self.piece.get_position().clone();
            let end = self.piece.get_position().clone() + offsets[idx];
            self.n[idx] = usize::MAX;
            if !self.is_in_bounds(end) {
                return None;
            }
            return Some(Move::new(start, end, self.piece.clone(), None, None, false, None, false));
        }
        None
    }
    
    fn next_bishop_offset(&mut self) -> Option<Move> {
        let min_n = self.n.iter().min().unwrap().clone();
        if min_n == usize::MAX {
            return None;
        }
        
        let idx = self.n.iter().position(|&r| r == min_n).unwrap();
        self.n[idx] += 1;
        let offset = self.offsets.as_ref().unwrap()[idx] * self.n[idx] as i32;
        let start = self.piece.get_position().clone();
        let end = self.piece.get_position().clone() + offset;
        if !self.is_in_bounds(end) {
            self.n[idx] = usize::MAX;
            return None;
        }
        return Some(Move::new(start, end, self.piece.clone(), None, None, false, None, false));
    }

    fn next_rook_offset(&mut self) -> Option<Move> {
        if self.buffer.len() > 0 {
            return self.buffer.pop()
        }
        let min_n = self.n.iter().min().unwrap().clone();
        if min_n == usize::MAX {
            return None;
        }
        
        let idx = self.n.iter().position(|&r| r == min_n).unwrap();
        self.n[idx] += 1;
        let offset = self.offsets.as_ref().unwrap()[idx] * self.n[idx] as i32;
        let start = self.piece.get_position().clone();
        let end = self.piece.get_position().clone() + offset;
        if !self.is_in_bounds(end) {
            self.n[idx] = usize::MAX;
            return None;
        }
        self.buffer.push(Move::new(start, end, self.piece.clone(), None, None, false, None, false));
        if !self.piece.has_moved() {
            let target_piece = self.state.get_piece_at(end);
            if (!target_piece.is_none()) && (target_piece.unwrap().get_piece_type() == PieceType::KING) {
                let target_piece = Some(target_piece.unwrap().clone());
                self.buffer.push(Move::new(start, end, self.piece.clone(), None, None, true, target_piece, false));
            }
        }
        self.buffer.pop()
    }
    
    fn next_queen_offset(&mut self) -> Option<Move> {
        let min_n = self.n.iter().min().unwrap().clone();
        if min_n == usize::MAX {
            return None;
        }
        
        let idx = self.n.iter().position(|&r| r == min_n).unwrap();
        self.n[idx] += 1;
        let offset = self.offsets.as_ref().unwrap()[idx] * self.n[idx] as i32;
        let start = self.piece.get_position().clone();
        let end = self.piece.get_position().clone() + offset;
        if !self.is_in_bounds(end) {
            self.n[idx] = usize::MAX;
            return None;
        }
        return Some(Move::new(start, end, self.piece.clone(), None, None, false, None, false));
    }
    
    fn next_king_offset(&mut self) -> Option<Move> {
        if self.buffer.len() > 0 {
            return self.buffer.pop()
        }
        let offsets: Vec<Vec2> =
            vec![
                Vec2::new( 1,  0),
                Vec2::new(-1,  0),
                Vec2::new( 0, -1),
                Vec2::new( 0,  1),
                Vec2::new( 1,  1),
                Vec2::new( 1, -1),
                Vec2::new(-1,  1),
                Vec2::new(-1, -1),
            ];
        for idx in 0..offsets.len() {
            if self.n[idx] != 0 {
                continue;
            }
            let start = self.piece.get_position().clone();
            let end = self.piece.get_position().clone() + offsets[idx];
            if !self.is_in_bounds(end) {
                self.n[idx] = usize::MAX;
                continue;
            }
            self.n[idx] = usize::MAX;
            
            self.buffer.push(Move::new(start, end, self.piece.clone(), None, None, false, None, false));
        }
        if !self.piece.has_moved() {
            let rooks = self.state.find(PieceType::ROOK, self.piece.get_color());
            for rook in rooks {
                if !rook.has_moved() {
                    let start = self.piece.get_position().clone();
                    let end = rook.get_position().clone();
                    let idx;
                    if start.x - end.x < 0 {
                        idx = 8;
                    }
                    else {
                        idx = 9;
                    }
                    if self.n[idx] == 0 {
                        self.buffer.push(Move::new(start, end, self.piece.clone(), None, None, true, Some(rook.clone()), false));
                        self.n[idx] = usize::MAX;
                    }
                }
            }
        }
        self.buffer.pop()
    }
    
    fn next_offset(&mut self) -> Option<Move> {
        match self.piece.get_piece_type() {
            PieceType::NULL => None,
            PieceType::PAWN => self.next_pawn_offset(),
            PieceType::KNIGHT => self.next_knight_offset(),
            PieceType::BISHOP => self.next_bishop_offset(),
            PieceType::ROOK => self.next_rook_offset(),
            PieceType::QUEEN => self.next_queen_offset(),
            PieceType::KING => self.next_king_offset(),
        }
    }
    
    fn is_color_correct(&self) -> bool {
        return self.piece.get_color() == self.state.to_move;
    }
    
    fn check_pawn_offset(&self, offset_move: &Move) -> bool {
        let offset = offset_move.end - offset_move.start;
        
        // check color based movement
        let mul = match self.piece.get_color() {
            PieceColor::WHITE => 1,
            PieceColor::BLACK => -1 };
        if offset.y * mul < 1 {
            return false;
        }
        
        // no double movement after move
        if (self.piece.has_moved()) && (offset.y.abs() > 1) {
            return false;
        }
        
        // check if promotion available
        if !self.state.config.promotion_lines.contains(&offset_move.end.y) {
            if !offset_move.promotion.is_none() {
                return false;
            }
        }
        else {
            if offset_move.promotion.is_none() {
                return false;
            }
        }
        
        // check attacks
        // enpassant TODO
        if offset.x != 0 {
            let attacked_piece = self.state.get_piece_at(offset_move.end);
            match attacked_piece {
                Some(_) => {},
                None => return false
            }
        }
        
        true
    }
    
    fn check_knight_offset(&self) -> bool {
        true
    }
    
    fn check_diagonal_offset(&mut self, offset_move: &Move) -> bool {
        let mut offset = offset_move.end - offset_move.start;
        if offset.abs().max_element() == 0 {
            return false;
        }
        offset /= offset.abs().max_element();
        
        // path blocked, dont generate more moves in that direction
        if !offset_move.target.is_none() {
            let local_offsets = self.offsets.clone().unwrap();
            for idx in 0..local_offsets.len() {
                if offset == local_offsets[idx] {
                    self.n[idx] = usize::MAX;
                    break;
                }
            }
        }
        
        // is diagonal?
        if offset.abs().x != offset.abs().y {
            return false;
        }
        
        true
    }

    fn check_horizontal_offset(&mut self, offset_move: &Move) -> bool {
        let mut offset = offset_move.end - offset_move.start;
        if offset.abs().max_element() == 0 {
            return false;
        }
        offset /= offset.abs().max_element();
        
        // path blocked, dont generate more moves in that direction
        if !offset_move.target.is_none() {
            let local_offsets = self.offsets.clone().unwrap();
            for idx in 0..local_offsets.len() {
                if offset == local_offsets[idx] {
                    self.n[idx] = usize::MAX;
                    break;
                }
            }
        }
        
        // is horizontal?
        if (offset.abs().x != 0) && (offset.abs().y != 0) {
            return false;
        }
        
        true
    }
    
    fn check_king_offset(&self, offset_move: &Move) -> bool {
        if offset_move.castling {
            let mut offset = offset_move.end - offset_move.start;
            offset /= offset.abs().x;
            let mut checked_point = offset_move.start + offset;
            loop {
                if checked_point == offset_move.end {
                    break;
                }
                let found_piece = self.state.get_piece_at(checked_point);
                if (!found_piece.is_none()) && (found_piece.unwrap().get_piece_type() != PieceType::ROOK) && (!found_piece.unwrap().has_moved()) {
                    return false;
                }
                checked_point += offset;
            }
        }
        true
    }
    
    pub fn next_pseudo(&mut self) -> Option<Move> {
        let offset_move = self.next_offset();
        
        // offset exists?
        if offset_move.is_none() {
            return None;
        }
        let mut offset_move = offset_move.unwrap();
        
        // correct color to move?
        if !self.is_color_correct() {
            return None;
        }

        // set target piece
        let attacked_piece = self.state.get_piece_at(offset_move.end);
        if !attacked_piece.is_none() {
            offset_move.target = Some(attacked_piece.unwrap().clone());
        }
        
        // correct offset?
        // is path blocked?
        let correct_offset: bool = match self.piece.get_piece_type() {
            PieceType::NULL => false,
            PieceType::PAWN => self.check_pawn_offset(&offset_move),
            PieceType::KNIGHT => self.check_knight_offset(),
            PieceType::BISHOP => self.check_diagonal_offset(&offset_move),
            PieceType::ROOK => self.check_horizontal_offset(&offset_move),
            PieceType::QUEEN => self.check_diagonal_offset(&offset_move) || self.check_horizontal_offset(&offset_move),
            PieceType::KING => self.check_king_offset(&offset_move),
        };
        if !correct_offset {
            return None;
        }
        
        // only pawns can promote
        if !offset_move.promotion.is_none() && (offset_move.piece.get_piece_type() != PieceType::PAWN) {
            return None;
        }
        
        // is target a friendly piece?
        if !offset_move.target.is_none() {
            if offset_move.piece.get_color() == offset_move.target.clone().unwrap().get_color() {
                if !offset_move.castling {
                    return None;
                }
            }
        }
        
        // castling
        
        // enpassant
        if offset_move.en_passant {
            if offset_move.start.x == offset_move.end.x {
                return None;
            }
            if self.state.previous_move.is_none() {
                return None;
            }
            let prev_move = self.state.previous_move.as_ref().unwrap().clone();
            if prev_move.piece.get_piece_type() != PieceType::PAWN {
                return None;
            }
            if (prev_move.start - prev_move.end).abs().y != 2 {
                return None;
            }
            if (offset_move.piece.get_position() - prev_move.piece.get_position()).abs().x != 1 {
                return None;
            }
        }
        
        Some(offset_move)
    }
    
    //pub fn next(&self) -> Move {
    //    
    //}
}