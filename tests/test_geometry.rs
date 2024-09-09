use quasar_chess_engine::quasar::geometry::Point;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point::new(3, 4);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 4);
    }

    #[test]
    fn test_point_addition() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let result = p1 + p2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn test_point_subtraction() {
        let p1 = Point::new(5, 7);
        let p2 = Point::new(2, 3);
        let result = p1 - p2;
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 4);
    }

    #[test]
    fn test_point_multiplication() {
        let p = Point::new(2, 3);
        let result = p * 3;
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 9);
    }

    #[test]
    fn test_point_division() {
        let p = Point::new(6, 9);
        let result = p / 3;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn test_point_operations_chaining() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = Point::new(1, 1);
        let result = (p1 + p2) * 2 - p3;
        assert_eq!(result.x, 7);
        assert_eq!(result.y, 11);
    }
}