use crate::quasar::geometry::Point;
use crate::quasar::pieces::{Piece, PieceType};
use crate::quasar::state::State;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Point,
    pub to: Point,
    pub piece: Piece,
    pub captured_piece: Option<Piece>,
    pub is_promotion: bool,
    pub promotion_type: Option<PieceType>,
    pub is_castling: bool,
    pub is_en_passant: bool,
    pub is_legal: bool,
}

impl Move {
    pub fn new(from: Point, to: Point, piece: Piece) -> Self {
        Move {
            from,
            to,
            piece,
            captured_piece: None,
            is_promotion: false,
            promotion_type: None,
            is_castling: false,
            is_en_passant: false,
            is_legal: false,
        }
    }

    pub fn with_capture(mut self, captured_piece: Piece) -> Self {
        self.captured_piece = Some(captured_piece);
        self
    }

    pub fn with_promotion(mut self, promotion_type: PieceType) -> Self {
        self.is_promotion = true;
        self.promotion_type = Some(promotion_type);
        self
    }

    pub fn set_castling(mut self) -> Self {
        self.is_castling = true;
        self
    }

    pub fn set_en_passant(mut self) -> Self {
        self.is_en_passant = true;
        self
    }

    pub fn is_legal(&mut self, state: &State) -> bool {
        if self.from == self.to || !self.is_piece_valid(state) {
            return false;
        }

        self.update_captured_piece(state);

        let basic_legal = self.is_basic_move_legal(state);
        let special_legal = self.is_special_move_legal(state);

        if !basic_legal || !special_legal {
            self.is_legal = false;
            return false;
        }

        self.is_legal = !self.leaves_king_in_check(state);
        self.is_legal
    }

    fn is_piece_valid(&mut self, state: &State) -> bool {
        match state.pieces.get_piece_at(self.from.x, self.from.y) {
            Some(piece) if piece.color == state.to_move => {
                self.piece = piece.clone();
                true
            }
            _ => false,
        }
    }

    fn update_captured_piece(&mut self, state: &State) {
        if let Some(piece_at_destination) = state.pieces.get_piece_at(self.to.x, self.to.y) {
            if piece_at_destination.color != self.piece.color {
                self.captured_piece = Some(piece_at_destination.clone());
            }
        }
    }

    fn is_basic_move_legal(&self, state: &State) -> bool {
        match self.piece.form {
            PieceType::Pawn => self.is_legal_pawn_move(state),
            PieceType::Knight => self.is_legal_knight_move(),
            PieceType::Bishop => self.is_legal_bishop_move(state),
            PieceType::Rook => self.is_legal_rook_move(state),
            PieceType::Queen => self.is_legal_queen_move(state),
            PieceType::King => self.is_legal_king_move(),
            PieceType::Null => false,
        }
    }

    fn is_special_move_legal(&self, state: &State) -> bool {
        if self.is_castling && !self.is_legal_castling(state) {
            return false;
        }
        if self.is_en_passant && !self.is_legal_en_passant(state) {
            return false;
        }
        true
    }

    fn leaves_king_in_check(&self, state: &State) -> bool {
        let new_state = state.make_move(self.clone());
        new_state.is_king_in_check(self.piece.color)
    }

    fn is_legal_pawn_move(&self, state: &State) -> bool {
        let direction = if self.piece.color { 1 } else { -1 };
        let forward = Point::new(0, direction);
        let double_forward = Point::new(0, 2 * direction);
        let attack_left = Point::new(-1, direction);
        let attack_right = Point::new(1, direction);

        let move_offset = self.to - self.from;

        match move_offset {
            offset if offset == forward => state.pieces.get_piece_at(self.to.x, self.to.y).is_none(),
            offset if offset == double_forward && !self.piece.moved => {
                let intermediate = self.from + forward;
                state.pieces.get_piece_at(intermediate.x, intermediate.y).is_none() &&
                state.pieces.get_piece_at(self.to.x, self.to.y).is_none()
            },
            offset if offset == attack_left || offset == attack_right => {
                state.pieces.get_piece_at(self.to.x, self.to.y).is_some() || self.is_en_passant
            },
            _ => false,
        }
    }

    fn is_legal_knight_move(&self) -> bool {
        let offsets = [
            Point::new(1, 2), Point::new(2, 1),
            Point::new(2, -1), Point::new(1, -2),
            Point::new(-1, -2), Point::new(-2, -1),
            Point::new(-2, 1), Point::new(-1, 2)
        ];
        offsets.contains(&(self.to - self.from))
    }

    fn is_legal_bishop_move(&self, state: &State) -> bool {
        let dx = (self.to.x - self.from.x).abs();
        let dy = (self.to.y - self.from.y).abs();
        dx == dy && self.is_path_clear(state)
    }

    fn is_legal_rook_move(&self, state: &State) -> bool {
        (self.from.x == self.to.x || self.from.y == self.to.y) && self.is_path_clear(state)
    }

    fn is_legal_queen_move(&self, state: &State) -> bool {
        (self.is_legal_bishop_move(state) || self.is_legal_rook_move(state)) && self.is_path_clear(state)
    }

    fn is_legal_king_move(&self) -> bool {
        let dx = (self.to.x - self.from.x).abs();
        let dy = (self.to.y - self.from.y).abs();
        dx <= 1 && dy <= 1
    }

    fn is_path_clear(&self, state: &State) -> bool {
        let dx = (self.to.x - self.from.x).signum();
        let dy = (self.to.y - self.from.y).signum();
        let mut current = self.from + Point::new(dx, dy);

        while current != self.to {
            if state.pieces.get_piece_at(current.x, current.y).is_some() {
                return false;
            }
            current = current + Point::new(dx, dy);
        }
        true
    }

    fn is_legal_castling(&self, state: &State) -> bool {
        let king = state.pieces.get_piece_at(self.from.x, self.from.y).unwrap();
        if king.moved {
            return false;
        }

        let (rook_x, rook_y) = if self.to.x > self.from.x {
            (7, self.from.y) // Kingside castling
        } else {
            (0, self.from.y) // Queenside castling
        };

        let rook = match state.pieces.get_piece_at(rook_x, rook_y) {
            Some(piece) if piece.form == PieceType::Rook && !piece.moved => piece,
            _ => return false,
        };

        self.is_castling_path_clear(state) && !self.is_castling_through_check(state)
    }

    fn is_castling_path_clear(&self, state: &State) -> bool {
        let step = if self.to.x > self.from.x { 1 } else { -1 };
        let mut x = self.from.x + step;
        while x != (if step > 0 { 7 } else { 0 }) {
            if state.pieces.get_piece_at(x, self.from.y).is_some() {
                return false;
            }
            x += step;
        }
        true
    }

    fn is_castling_through_check(&self, state: &State) -> bool {
        let step = if self.to.x > self.from.x { 1 } else { -1 };
        let mut x = self.from.x;
        while x != self.to.x {
            let mut test_move = Move::new(self.from, Point::new(x, self.from.y), self.piece.clone());
            if state.is_king_in_check(self.piece.color) || !test_move.is_legal(state) {
                return true;
            }
            x += step;
        }
        false
    }

    fn is_legal_en_passant(&self, state: &State) -> bool {
        if self.piece.form != PieceType::Pawn {
            return false;
        }

        if let Some(last_move) = &state.last_move {
            if last_move.piece.form != PieceType::Pawn || (last_move.to.y as i8 - last_move.from.y as i8).abs() != 2 {
                return false;
            }

            let expected_y = if self.piece.color { 4 } else { 3 };
            if self.from.y != expected_y {
                return false;
            }

            let expected_to_y = if self.piece.color { 5 } else { 2 };
            if self.to.y != expected_to_y || self.to.x != last_move.to.x {
                return false;
            }

            true
        } else {
            false
        }
    }
}
