use crate::quasar::state::State;
use crate::quasar::geometry::Point;
use crate::quasar::pieces::{Piece, PieceType, PieceList};

// pub fn classical_icn(icn: String) {//-> State<i128, 32> {
//     // let mut list;
//     let move_count: usize;
//     let to_move;
    
//     let sections: Vec<&str> = icn.split(" ").collect();
//     println!("{:?}", sections);
//     println!();
//     println!("{}", sections[24]);
    
//     if sections[20] == "w" {
//         to_move = true;
//     }
//     else {
//         to_move = false;
//     }
    
//     move_count = sections[22].parse().unwrap();
//     println!("{}", move_count);
//     println!("{}", to_move);
    
//     // State::new(list, move_count, to_move)
// }

pub fn parse_fen(fen: &str) -> Result<State, String> {
    let mut pieces = Vec::new();
    let move_count;
    let to_move;

    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() != 6 {
        return Err("Invalid FEN string: incorrect number of fields".to_string());
    }

    // Parse piece positions
    let ranks: Vec<&str> = parts[0].split('/').collect();
    if ranks.len() != 8 {
        return Err("Invalid FEN string: incorrect number of ranks".to_string());
    }

    for (rank, rank_str) in ranks.iter().enumerate() {
        let mut file = 0;
        for c in rank_str.chars() {
            if let Some(digit) = c.to_digit(10) {
                file += digit as usize;
            } else {
                let piece_type = match c.to_ascii_lowercase() {
                    'p' => PieceType::Pawn,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    _ => return Err(format!("Invalid piece type: {}", c)),
                };

                let color = c.is_ascii_uppercase();
                let position = Point { x: (file + 1) as i128, y: (8 - rank) as i128 };

                pieces.push(Piece::new(piece_type, position, color));

                file += 1;
            }
        }
    }

    // Parse active color
    to_move = match parts[1] {
        "w" => true,
        "b" => false,
        _ => return Err("Invalid active color in FEN".to_string()),
    };

    // Parse halfmove clock
    move_count = parts[4].parse().map_err(|_| "Invalid halfmove clock in FEN".to_string())?;

    let piece_list = PieceList::new(pieces);
    Ok(State::new(piece_list, move_count, to_move))
}
