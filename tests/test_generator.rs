use quasar_chess_engine::quasar::generator::MoveGenerator;
use quasar_chess_engine::quasar::pieces::{Piece, PieceType};
use quasar_chess_engine::quasar::geometry::Point;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_generator_new() {
        let piece = Piece::new(PieceType::Knight, Point::new(0, 0), true);
        let generator = MoveGenerator::new(&piece);
        assert_eq!(generator.current_position, Point::new(0, 0));
        assert_eq!(generator.current_offset_index, 0);
        assert_eq!(generator.current_multiplier, 1);
    }

    #[test]
    fn test_move_generator_next() {
        let piece = Piece::new(PieceType::Knight, Point::new(0, 0), true);
        let mut generator = MoveGenerator::new(&piece);

        // Knight's moves from (0, 0)
        let expected_moves = vec![
            Point::new(1, 2),
            Point::new(2, 1),
            Point::new(2, -1),
            Point::new(1, -2),
            Point::new(-1, -2),
            Point::new(-2, -1),
            Point::new(-2, 1),
            Point::new(-1, 2),
        ];

        for expected_move in expected_moves {
            assert_eq!(generator.next(), Some(expected_move));
        }
    }

    #[test]
    fn test_move_generator_sliding_piece() {
        let piece = Piece::new(PieceType::Bishop, Point::new(0, 0), true);
        let mut generator = MoveGenerator::new(&piece);

        // First 8 moves of a Bishop from (0, 0)
        let expected_moves = vec![
            Point::new(1, 1),
            Point::new(1, -1),
            Point::new(-1, -1),
            Point::new(-1, 1),
            Point::new(2, 2),
            Point::new(2, -2),
            Point::new(-2, -2),
            Point::new(-2, 2),
        ];

        for expected_move in expected_moves {
            assert_eq!(generator.next(), Some(expected_move));
        }
    }

    #[test]
    fn test_move_generator_exhaustion() {
        let piece = Piece::new(PieceType::King, Point::new(0, 0), true);
        let mut generator = MoveGenerator::new(&piece);

        // Exhaust all 8 moves of the King
        for _ in 0..8 {
            assert!(generator.next().is_some());
        }

        // The 9th call should return None
        assert_eq!(generator.next(), None);
    }
}
