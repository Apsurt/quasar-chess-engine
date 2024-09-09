use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

impl Point {
    pub fn new(x: i128, y: i128) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn checked_add(self, other: Point) -> Option<Point> {
        Some(Point {
            x: self.x.checked_add(other.x)?,
            y: self.y.checked_add(other.y)?,
        })
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Into<i128>> ops::Add<T> for Point {
    type Output = Point;

    fn add(self, other: T) -> Point {
        let value = other.into();
        Point {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Into<i128>> ops::Sub<T> for Point {
    type Output = Point;

    fn sub(self, other: T) -> Point {
        let value = other.into();
        Point {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl<T: Into<i128>> ops::Mul<T> for Point {
    type Output = Point;

    fn mul(self, scalar: T) -> Point {
        let value = scalar.into();
        Point {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl<T: Into<i128>> ops::Div<T> for Point {
    type Output = Point;

    fn div(self, scalar: T) -> Point {
        let value = scalar.into();
        Point {
            x: self.x / value,
            y: self.y / value,
        }
    }
}