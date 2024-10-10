use glam::IVec2 as Vec2;

#[derive(Debug, Clone)]
pub struct Config {
    pub boundaries: [Vec2;2],
    pub promotion_lines: Vec<i32>,
}

impl Config {
    pub fn new(boundaries: [Vec2;2] , promotion_lines: Vec<i32>) -> Config {
        Config { boundaries, promotion_lines }
    }
    
    pub fn default() -> Config {
        let boundaries = [Vec2::new(i32::MIN, i32::MAX), Vec2::new(i32::MAX, i32::MIN)];
        let promotion_lines = vec![1,8];
        Config { boundaries, promotion_lines }
    }
}