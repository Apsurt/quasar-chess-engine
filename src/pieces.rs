use glam::IVec2 as Vec2;
use core::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    BLACK,
    WHITE
}

impl PieceColor {
    pub fn from_bool(value: bool) -> PieceColor {
        match value {
            false => PieceColor::BLACK,
            true => PieceColor::WHITE
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    NULL,
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

impl PieceType {
    pub fn from_u8(value: u8) -> PieceType {
        match value {
            0 => PieceType::NULL,
            1 => PieceType::PAWN,
            2 => PieceType::KNIGHT,
            3 => PieceType::BISHOP,
            4 => PieceType::ROOK,
            5 => PieceType::QUEEN,
            6 => PieceType::KING,
            _ => PieceType::NULL,
        }
    }
}

pub fn name_to_type(name: String) -> PieceType {
    let name: &str = &format!("{}", name.to_lowercase());
    match name {
        "pawn" => PieceType::PAWN,
        "knight" => PieceType::KNIGHT,
        "bishop" => PieceType::BISHOP,
        "rook" => PieceType::ROOK,
        "queen" => PieceType::QUEEN,
        "king" => PieceType::KING,
        _ => PieceType::NULL,
    }
}

pub fn type_to_name(piece_type: PieceType) -> String {
    match piece_type {
        PieceType::NULL => "null".to_owned(),
        PieceType::PAWN => "pawn".to_owned(),
        PieceType::KNIGHT => "knight".to_owned(),
        PieceType::BISHOP => "bishop".to_owned(),
        PieceType::ROOK => "rook".to_owned(),
        PieceType::QUEEN => "queen".to_owned(),
        PieceType::KING => "king".to_owned(),
    }
}

pub fn name_to_symbol(name: String) -> char {
    let name: &str = &format!("{}", name.to_lowercase());
    match name {
        "pawn" => 'p',
        "knight" => 'n',
        "bishop" => 'b',
        "rook" => 'r',
        "queen" => 'q',
        "king" => 'k',
        _ => 'x',
    }
}

pub fn symbol_to_name(symbol: char) -> String {
    let symbol = symbol.to_ascii_lowercase();
    match symbol {
        'p' => "pawn".to_owned(),
        'n' => "knight".to_owned(),
        'b' => "bishop".to_owned(),
        'r' => "rook".to_owned(),
        'q' => "queen".to_owned(),
        'k' => "king".to_owned(),
        _ => "null".to_owned(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    color: PieceColor,
    has_moved: bool,
    is_alive: bool,
    piece_type: PieceType,
    position: Vec2,
}

impl Piece {
    pub fn new(piece_color: PieceColor, piece_type: PieceType, position: Vec2) -> Piece {
        Piece {color: piece_color, has_moved: false, is_alive: true, piece_type, position}
    }
    
    pub fn get_color(&self) -> PieceColor {
        return self.color
    }
    
    pub fn has_moved(&self) -> bool {
        return self.has_moved
    }
    
    pub fn moved(&mut self) {
        self.has_moved = true
    }
    
    pub fn is_alive(&self) -> bool {
        return self.is_alive
    }
    
    pub fn capture(&mut self) {
        self.is_alive = false
    }
    
    pub fn get_piece_type(&self) -> PieceType {
        return self.piece_type
    }
    
    pub fn get_name(&self) -> String {
        type_to_name(self.get_piece_type())
    }
    
    pub fn get_symbol(&self) -> char {
        let symbol = name_to_symbol(type_to_name(self.get_piece_type()));
        match self.get_color() {
            PieceColor::BLACK => symbol,
            PieceColor::WHITE => symbol.to_ascii_uppercase()
        }
    }
    
    pub fn get_position(&self) -> &Vec2 {
        return &self.position
    }
    
    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
       }
}