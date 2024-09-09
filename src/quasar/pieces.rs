use strum_macros::EnumIter;
use crate::quasar::geometry::Point;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum PieceType {
    Null,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn is_sliding(&self) -> bool {
        matches!(self, PieceType::Bishop | PieceType::Rook | PieceType::Queen)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub form: PieceType,
    pub position: Point,
    pub color: bool,
    pub sliding: bool,
    pub alive: bool,
    pub moved: bool,
    pub offsets: Vec<Point>,
    pub is_en_passant_target: bool,
}

impl Piece {
    pub fn new(form: PieceType, position: Point, color: bool) -> Self {
        Piece {
            form,
            position,
            color,
            sliding: form.is_sliding(),
            alive: true,
            moved: false,
            offsets: Self::get_offsets(form, color),
            is_en_passant_target: false,
        }
    }

    fn get_offsets(piece_type: PieceType, color: bool) -> Vec<Point> {
        match piece_type {
            PieceType::Pawn => Self::pawn_offsets(color),
            PieceType::Knight => Self::knight_offsets(),
            PieceType::Bishop => Self::bishop_offsets(),
            PieceType::Rook => Self::rook_offsets(),
            PieceType::Queen | PieceType::King => Self::queen_king_offsets(),
            PieceType::Null => vec![],
        }
    }

    fn pawn_offsets(color: bool) -> Vec<Point> {
        let direction = if color { 1 } else { -1 };
        vec![
            Point::new(0, direction),     // Move forward
            Point::new(0, 2 * direction), // Double forward
            Point::new(-1, direction),    // Attack left
            Point::new(1, direction),     // Attack right
        ]
    }

    fn knight_offsets() -> Vec<Point> {
        vec![
            Point::new(1, 2), Point::new(2, 1),
            Point::new(2, -1), Point::new(1, -2),
            Point::new(-1, -2), Point::new(-2, -1),
            Point::new(-2, 1), Point::new(-1, 2),
        ]
    }

    fn bishop_offsets() -> Vec<Point> {
        vec![
            Point::new(1, 1), Point::new(1, -1),
            Point::new(-1, -1), Point::new(-1, 1),
        ]
    }

    fn rook_offsets() -> Vec<Point> {
        vec![
            Point::new(0, 1), Point::new(1, 0),
            Point::new(0, -1), Point::new(-1, 0),
        ]
    }

    fn queen_king_offsets() -> Vec<Point> {
        vec![
            Point::new(0, 1), Point::new(1, 1),
            Point::new(1, 0), Point::new(1, -1),
            Point::new(0, -1), Point::new(-1, -1),
            Point::new(-1, 0), Point::new(-1, 1),
        ]
    }

    pub fn update_offsets(&mut self) {
        if let PieceType::Pawn = self.form {
            let (forward, attack_left, attack_right) = if self.color {
                (Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1))
            } else {
                (Point::new(0, -1), Point::new(-1, -1), Point::new(1, -1))
            };

            self.offsets = if self.moved {
                vec![forward, attack_left, attack_right]
            } else {
                let double_forward = Point::new(0, if self.color { 2 } else { -2 });
                vec![forward, double_forward, attack_left, attack_right]
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct PieceList {
    pub list: Vec<Piece>,
    white_count: usize,
    black_count: usize,
    white_index: usize,
}

impl PieceList {
    pub fn new(mut pieces: Vec<Piece>) -> Self {
        pieces.sort_by_key(|p| (!p.color, p.form as u8));
        let white_count = pieces.iter().filter(|p| p.color).count();
        let black_count = pieces.len() - white_count;
        
        PieceList {
            list: pieces,
            white_count,
            black_count,
            white_index: white_count,
        }
    }

    pub fn get_all_pieces(&self) -> &[Piece] {
        &self.list
    }

    pub fn get_white_pieces(&self) -> &[Piece] {
        &self.list[..self.white_index]
    }

    pub fn get_black_pieces(&self) -> &[Piece] {
        &self.list[self.white_index..]
    }

    pub fn get_pieces_by_type(&self, piece_type: PieceType) -> Vec<&Piece> {
        self.list.iter().filter(|p| p.form == piece_type).collect()
    }

    pub fn get_white_pieces_by_type(&self, piece_type: PieceType) -> Vec<&Piece> {
        self.get_white_pieces().iter().filter(|p| p.form == piece_type).collect()
    }

    pub fn get_black_pieces_by_type(&self, piece_type: PieceType) -> Vec<&Piece> {
        self.get_black_pieces().iter().filter(|p| p.form == piece_type).collect()
    }

    pub fn get_alive_pieces(&self) -> Vec<&Piece> {
        self.list.iter().filter(|p| p.alive).collect()
    }

    pub fn get_sliding_pieces(&self) -> Vec<&Piece> {
        self.list.iter().filter(|p| p.sliding).collect()
    }

    pub fn get_unmoved_pieces(&self) -> Vec<&Piece> {
        self.list.iter().filter(|p| !p.moved).collect()
    }

    pub fn get_piece_at(&self, x: i128, y: i128) -> Option<&Piece> {
        self.list.iter().find(|p| p.position.x == x && p.position.y == y)
    }

    pub fn get_piece_mut_at(&mut self, x: i128, y: i128) -> Option<&mut Piece> {
        self.list.iter_mut().find(|p| p.position.x == x && p.position.y == y)
    }

    pub fn get_white_king(&self) -> &Piece {
        &self.list[self.white_index - 1]
    }

    pub fn get_black_king(&self) -> &Piece {
        self.list.last().unwrap()
    }

    pub fn get_board_boundaries(&self) -> (i128, i128, i128, i128) {
        let (min_x, max_x, min_y, max_y) = self.list.iter().fold(
            (i128::MAX, i128::MIN, i128::MAX, i128::MIN),
            |(min_x, max_x, min_y, max_y), p| {
                (
                    min_x.min(p.position.x),
                    max_x.max(p.position.x),
                    min_y.min(p.position.y),
                    max_y.max(p.position.y),
                )
            },
        );
        (min_x, max_x, min_y, max_y)
    }
    
    pub fn get_en_passant_target(&self) -> Option<(i128, i128)> {
        self.list.iter()
            .find(|p| p.form == PieceType::Pawn && p.is_en_passant_target)
            .map(|p| (p.position.x, p.position.y))
    }

    pub fn set_en_passant_target(&mut self, x: i128, y: i128) {
        if let Some(piece) = self.get_piece_mut_at(x, y) {
            piece.is_en_passant_target = true;
        }
    }

    pub fn clear_en_passant_target(&mut self) {
        for piece in &mut self.list {
            piece.is_en_passant_target = false;
        }
    }
}