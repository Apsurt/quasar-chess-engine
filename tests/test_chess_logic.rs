use quasar_chess_engine::quasar::state::State;
use quasar_chess_engine::quasar::geometry::Point;

#[test]
fn test_chess_logic_performance() {
    let expected_positions = vec![20, 400, 8902, 197281, 4865609];
    let max_depth = 5;

    for depth in 1..=max_depth {
        println!("Starting depth {}", depth);
        let positions = count_positions(depth);
        println!("Depth {}: {} positions", depth, positions);
        if positions != expected_positions[depth - 1] {
            panic!("Mismatch at depth {}. Expected: {}, Got: {}", depth, expected_positions[depth - 1], positions);
        }
    }
}

fn count_positions(max_depth: usize) -> usize {
    let mut count = 0;
    let initial_state = State::default();
    count_positions_recursive(&initial_state, 0, max_depth, &mut count);
    count
}

fn count_positions_recursive(state: &State, current_depth: usize, max_depth: usize, count: &mut usize) {
    if current_depth == max_depth {
        *count += 1;
        if *count % 1000 == 0 {
            println!("Counted {} positions at depth {}", *count, max_depth);
        }
        return;
    }

    let legal_moves = state.get_legal_moves();
    println!("Depth {}: {} legal moves", current_depth, legal_moves.len());

    for (i, move_) in legal_moves.iter().enumerate() {
        if i % 100 == 0 {
            println!("Depth {}: Processing move {} of {}", current_depth, i + 1, legal_moves.len());
        }
        if is_within_normal_chess_boundaries(&move_.to) {
            let new_state = state.make_move(move_.clone());
            count_positions_recursive(&new_state, current_depth + 1, max_depth, count);
        }
    }
}

fn is_within_normal_chess_boundaries(point: &Point) -> bool {
    point.x >= 1 && point.x <= 8 && point.y >= 1 && point.y <= 8
}
