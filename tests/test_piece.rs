use quasar::pieces::*;
use glam::IVec2 as Vec2;

#[test]
fn test_piece_struct() {
    let piece = Piece::new(PieceColor::WHITE, PieceType::PAWN, Vec2::ZERO);
    
    assert_eq!(piece.get_color(), PieceColor::WHITE);
    assert_eq!(piece.get_piece_type(), PieceType::PAWN);
    assert_eq!(piece.get_position(), &Vec2::ZERO);
    assert_eq!(piece.get_name(), "pawn");
    assert_eq!(piece.get_symbol(), 'P');
    assert_eq!(piece.has_moved(), false);
    assert_eq!(piece.is_alive(), true);
}

#[test]
fn test_set_position() {
    let mut piece = Piece::new(PieceColor::WHITE, PieceType::PAWN, Vec2::ZERO);
    assert_eq!(piece.get_position(), &Vec2::ZERO);
    piece.set_position(Vec2::ONE);
    assert_eq!(piece.get_position(), &Vec2::ONE);
}

#[test]
fn test_capture() {
    let mut piece = Piece::new(PieceColor::WHITE, PieceType::PAWN, Vec2::ZERO);
    assert_eq!(piece.is_alive(), true);
    piece.capture();
    assert_eq!(piece.is_alive(), false);
}

#[test]
fn tet_moved() {
    let mut piece = Piece::new(PieceColor::WHITE, PieceType::PAWN, Vec2::ZERO);
    assert_eq!(piece.has_moved(), false);
    piece.moved();
    assert_eq!(piece.has_moved(), true);
}

#[test]
fn test_color_from_bool() {
    assert_eq!(PieceColor::BLACK, PieceColor::from_bool(false));
    assert_eq!(PieceColor::WHITE, PieceColor::from_bool(true));
}

#[test]
fn test_type_from_int() {
    assert_eq!(PieceType::NULL, PieceType::from_u8(0));
    assert_eq!(PieceType::PAWN, PieceType::from_u8(1));
    assert_eq!(PieceType::KNIGHT, PieceType::from_u8(2));
    assert_eq!(PieceType::BISHOP, PieceType::from_u8(3));
    assert_eq!(PieceType::ROOK, PieceType::from_u8(4));
    assert_eq!(PieceType::QUEEN, PieceType::from_u8(5));
    assert_eq!(PieceType::KING, PieceType::from_u8(6));
    assert_eq!(PieceType::NULL, PieceType::from_u8(255));
}

#[test]
fn test_name_to_type() {
    assert_eq!(name_to_type("null".to_owned()), PieceType::NULL);
    assert_eq!(name_to_type("pAwn".to_owned()), PieceType::PAWN);
    assert_eq!(name_to_type("knIgHt".to_owned()), PieceType::KNIGHT);
    assert_eq!(name_to_type("BishoP".to_owned()), PieceType::BISHOP);
    assert_eq!(name_to_type("ROOK".to_owned()), PieceType::ROOK);
    assert_eq!(name_to_type("QUeen".to_owned()), PieceType::QUEEN);
    assert_eq!(name_to_type("kINg".to_owned()), PieceType::KING);
}

#[test]
fn test_type_to_name() {
    assert_eq!("null", type_to_name(PieceType::NULL));
    assert_eq!("pawn", type_to_name(PieceType::PAWN));
    assert_eq!("knight", type_to_name(PieceType::KNIGHT));
    assert_eq!("bishop", type_to_name(PieceType::BISHOP));
    assert_eq!("rook", type_to_name(PieceType::ROOK));
    assert_eq!("queen", type_to_name(PieceType::QUEEN));
    assert_eq!("king", type_to_name(PieceType::KING));
}

#[test]
fn test_name_to_symbol() {
    assert_eq!(name_to_symbol("null".to_owned()), 'x');
    assert_eq!(name_to_symbol("pAwn".to_owned()), 'p');
    assert_eq!(name_to_symbol("knIgHt".to_owned()), 'n');
    assert_eq!(name_to_symbol("BishoP".to_owned()), 'b');
    assert_eq!(name_to_symbol("ROOK".to_owned()), 'r');
    assert_eq!(name_to_symbol("QUeen".to_owned()), 'q');
    assert_eq!(name_to_symbol("kINg".to_owned()), 'k');
    assert_eq!(name_to_symbol("sdjfhksdbfjsb".to_owned()), 'x');
}

#[test]
fn test_symbol_to_name() {
    assert_eq!(symbol_to_name('p'), "pawn");
    assert_eq!(symbol_to_name('N'), "knight");
    assert_eq!(symbol_to_name('b'), "bishop");
    assert_eq!(symbol_to_name('R'), "rook");
    assert_eq!(symbol_to_name('q'), "queen");
    assert_eq!(symbol_to_name('K'), "king");
    assert_eq!(symbol_to_name('x'), "null")
}