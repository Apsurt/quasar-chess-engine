use crate::quasar::geometry::Point;
use crate::quasar::pieces::Piece;

pub struct MoveGenerator {
    pub current_position: Point,
    offsets: Vec<Point>,
    pub current_offset_index: usize,
    pub current_multiplier: i8,
    is_sliding: bool,
}

impl MoveGenerator {
    pub fn new(piece: &Piece) -> Self {
        MoveGenerator {
            current_position: piece.position,
            offsets: piece.offsets.clone(),
            current_offset_index: 0,
            current_multiplier: 1,
            is_sliding: piece.form.is_sliding(),
        }
    }

    pub fn next(&mut self) -> Option<Point> {
        if self.current_offset_index >= self.offsets.len() {
            if !self.is_sliding {
                return None;
            }
            self.current_offset_index = 0;
            self.current_multiplier = self.current_multiplier.saturating_add(1);
        }

        let offset = self.offsets[self.current_offset_index];
        let scaled_offset = Point::new(
            offset.x.saturating_mul(self.current_multiplier as i128),
            offset.y.saturating_mul(self.current_multiplier as i128)
        );
        let new_point = self.current_position.checked_add(scaled_offset)?;
        self.current_offset_index += 1;

        Some(new_point)
    }
}
