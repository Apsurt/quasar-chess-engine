use std::time::Instant;

use glam::IVec2 as Vec2;
use quasar::state::State;
use quasar::moves::Generator;

fn main() {
    let state = State::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned());
    println!("{}", state);
    
    let piece = state.get_piece_at(Vec2::new(4, 1)).unwrap().clone();
    let mut gen = Generator::new(piece, state);
    let mut i: i128 = 0;
    let start = Instant::now();
    loop {
        if i > 100 {
            break;
        }
        i += 1;
        let piece_move = gen.next_pseudo();
        match piece_move {
            None => {},
            Some(_) => println!("{} {} {} {:?}",
                piece_move.as_ref().unwrap().piece,
                piece_move.as_ref().unwrap().start,
                piece_move.as_ref().unwrap().end,
                piece_move.as_ref().unwrap().promotion,
            )
        }
    }
    println!("{:?}", start.elapsed())
}
