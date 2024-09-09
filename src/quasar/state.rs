use crate::quasar::pieces::{PieceList, PieceType, Piece};
use crate::quasar::moves::Move;
use crate::quasar::generator::MoveGenerator;
use crate::quasar::geometry::Point;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct State {
    pub pieces: PieceList,
    pub move_count: usize,
    pub to_move: bool,
    pub previous_state: Option<Rc<State>>,
    pub last_move: Option<Move>,
}

impl State {
    pub fn new(pieces: PieceList, move_count: usize, to_move: bool) -> Self {
        State {
            pieces,
            move_count,
            to_move,
            previous_state: None,
            last_move: None,
        }
    }

    pub fn default() -> Self {
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        crate::quasar::parser::parse_fen(default_fen)
            .unwrap_or_else(|e| panic!("Failed to create default state: {}", e))
    }

    pub fn make_move(&self, move_: Move) -> Self {
        let mut new_pieces = self.pieces.clone();
        self.update_piece_position(&mut new_pieces, &move_);
        self.handle_capture(&mut new_pieces, &move_);
        self.handle_special_moves(&mut new_pieces, &move_);

        State {
            pieces: new_pieces,
            move_count: self.move_count + 1,
            to_move: !self.to_move,
            previous_state: Some(Rc::new(self.clone())),
            last_move: Some(move_),
        }
    }

    fn update_piece_position(&self, new_pieces: &mut PieceList, move_: &Move) {
        if let Some(mut piece) = new_pieces.get_piece_mut_at(move_.from.x, move_.from.y) {
            piece.position = move_.to;
            piece.moved = true;
            if move_.is_promotion {
                piece.form = move_.promotion_type.unwrap_or(piece.form);
            }
        }
    }

    fn handle_capture(&self, new_pieces: &mut PieceList, move_: &Move) {
        if move_.captured_piece.is_some() {
            if let Some(piece) = new_pieces.get_piece_mut_at(move_.to.x, move_.to.y) {
                piece.alive = false;
            }
        }
    }

    fn handle_special_moves(&self, new_pieces: &mut PieceList, move_: &Move) {
        if move_.is_castling {
            self.handle_castling(new_pieces, move_);
        }

        if move_.is_en_passant {
            self.handle_en_passant(new_pieces, move_);
        }

        self.update_en_passant_target(new_pieces, move_);
    }

    fn handle_castling(&self, new_pieces: &mut PieceList, move_: &Move) {
        let (rook_from, rook_to) = if move_.to.x > move_.from.x {
            (Point::new(7, move_.from.y), Point::new(5, move_.from.y)) // Kingside castling
        } else {
            (Point::new(0, move_.from.y), Point::new(3, move_.from.y)) // Queenside castling
        };
        if let Some(rook) = new_pieces.get_piece_mut_at(rook_from.x, rook_from.y) {
            rook.position = rook_to;
            rook.moved = true;
        }
    }

    fn handle_en_passant(&self, new_pieces: &mut PieceList, move_: &Move) {
        let captured_pawn_y = if self.to_move { move_.to.y - 1 } else { move_.to.y + 1 };
        if let Some(pawn) = new_pieces.get_piece_mut_at(move_.to.x, captured_pawn_y) {
            pawn.alive = false;
        }
    }

    fn update_en_passant_target(&self, new_pieces: &mut PieceList, move_: &Move) {
        new_pieces.clear_en_passant_target();
        if let Some(piece) = new_pieces.get_piece_at(move_.to.x, move_.to.y) {
            if piece.form == PieceType::Pawn && (move_.to.y as i8 - move_.from.y as i8).abs() == 2 {
                let en_passant_y = (move_.from.y + move_.to.y) / 2;
                new_pieces.set_en_passant_target(move_.to.x, en_passant_y);
            }
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.pieces.get_all_pieces()
            .iter()
            .filter(|piece| piece.color == self.to_move && piece.alive)
            .flat_map(|piece| self.generate_legal_moves_for_piece(piece))
            .collect()
    }

    pub fn is_king_in_check(&self, color: bool) -> bool {
        let king = if color {
            self.pieces.get_white_king()
        } else {
            self.pieces.get_black_king()
        };

        self.pieces.get_all_pieces()
            .iter()
            .any(|piece| {
                piece.color != color && piece.alive &&
                Move::new(piece.position, king.position, piece.clone()).is_legal(self)
            })
    }

    pub fn is_checkmate(&self, color: bool) -> bool {
        self.is_king_in_check(color) && self.get_legal_moves().is_empty()
    }

    fn generate_legal_moves_for_piece(&self, piece: &Piece) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        let mut generator = MoveGenerator::new(piece);

        while let Some(to) = generator.next() {
            let mut move_ = Move::new(piece.position, to, piece.clone());
            if move_.is_legal(self) {
                legal_moves.push(move_);
            }
        }

        legal_moves
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.pieces.get_board_boundaries();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if let Some(piece) = self.pieces.get_piece_at(x, y) {
                    if !piece.alive {
                        write!(f, " ")?;
                        continue;
                    }
                    let symbol = match (piece.form, piece.color) {
                        (PieceType::Pawn, true) => "♟",
                        (PieceType::Knight, true) => "♞",
                        (PieceType::Bishop, true) => "♝",
                        (PieceType::Rook, true) => "♜",
                        (PieceType::Queen, true) => "♛",
                        (PieceType::King, true) => "♚",
                        (PieceType::Pawn, false) => "♙",
                        (PieceType::Knight, false) => "♘",
                        (PieceType::Bishop, false) => "♗",
                        (PieceType::Rook, false) => "♖",
                        (PieceType::Queen, false) => "♕",
                        (PieceType::King, false) => "♔",
                        _ => " ",
                    };
                    write!(f, "{}", symbol)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}