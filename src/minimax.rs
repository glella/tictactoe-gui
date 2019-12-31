
use crate::game::{Board, Player, GameState};
use std::cmp;

const MAX: i32 = 1000;
const MIN: i32 = -1000;

// Added Alpha Beta Pruning to Minimax 
// https://www.geeksforgeeks.org/minimax-algorithm-in-game-theory-set-4-alpha-beta-pruning/
pub fn minimax_abpruning(board: Board, player: Player, mut alpha: i32, mut beta: i32) -> i32 { 

	if board.is_ended() {
		let game_state = board.get_winner();
		match &game_state {
			GameState::GameWon { player, cells: _ } => {
        		match &player {
        			Player::O	=> return 10,	// AI won
					Player::X  	=> return -10,	// PLAYER won
        		}
            },
            _ => return 0,	// draw
		}
	}

	if player == Player::O { 					// Simulate AI (Max of Minimax)
		let mut best_move = MIN;
		let possible_moves = board.get_actions();
		for amove in possible_moves {
			let mut board_copy = board.clone(); // copy board
			board_copy.perform_action(amove);	// perform action in board copy
			let result = minimax_abpruning(board_copy, player.opponent(), alpha, beta);
			best_move = cmp::max(best_move, result);
			alpha = cmp::max(alpha, best_move);
			// Alpha Beta Pruning 
            if beta <= alpha {
                break; 
            }
		}
		return best_move;

	} else {									// Simulate Human (Mini of Minimax)
		
		let mut best_move = MAX;
		let possible_moves = board.get_actions();
		for amove in possible_moves {
			let mut board_copy = board.clone(); // copy board
			board_copy.perform_action(amove);	// perform action in board copy
			let result = minimax_abpruning(board_copy, player.opponent(), alpha, beta);
			best_move = cmp::min(best_move, result);
			beta = cmp::min(beta, best_move);
			// Alpha Beta Pruning 
            if beta <= alpha {
                break; 
            }
		}
		return best_move;
	}

}

pub fn find_best_move(board: Board, player: Player) -> (i32, i32) {

	let mut best_score = -1000;
	let mut best_move = (-1, -1);

	let possible_moves = board.get_actions();
	for amove in possible_moves {
		let mut board_copy = board.clone(); // copy board
		board_copy.perform_action(amove);	// perform action in board copy
		let score = minimax_abpruning(board_copy, player.opponent(), MIN, MAX);
		if score > best_score {
			best_score = score;
			best_move = amove;
		}
	}
	return best_move;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_minimax() {
        let mut board = Board::new(Player::O);
        board.fields = [
            [Some(Player::X),    None,     			Some(Player::O)],
            [Some(Player::X),    None,              None],
            [Some(Player::O),    Some(Player::X),   None]
        ];

        assert_eq!(find_best_move(board, board.next_player), (1,1));
    }

}


