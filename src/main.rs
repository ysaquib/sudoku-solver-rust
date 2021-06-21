pub mod sudoku_solver;

pub fn sudoku_board() -> sudoku_solver::solver::SudokuBoard {
    [
        [0, 0, 9, 1, 5, 0, 6, 0, 0],
        [0, 0, 7, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 9, 0, 0, 0, 0, 5],
        [0, 0, 8, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 7, 9, 8, 0, 0, 0],
        [5, 9, 0, 0, 0, 0, 0, 0, 1],
        [9, 6, 2, 0, 0, 0, 1, 0, 0],
        [0, 1, 5, 0, 2, 4, 0, 0, 0],
        [7, 0, 4, 6, 0, 9, 2, 0, 0],
    ]
}

fn main() {
    let solved = sudoku_solver::solver::solve(sudoku_board());
    for row in 0..9 {
        println!("{:?}", solved[row]);
    }
}

#[cfg(test)]
pub mod test {
    use super::sudoku_solver::solver;
    use crate::sudoku_board;

    #[test]
    fn test_not_solved() {
        let board = sudoku_board();
        assert!(!solver::is_solved(board));
    }

    #[test]
    fn invalid_guesses() {
        let board = sudoku_board();
        assert!(!solver::is_valid_guess(board, 0, 0, 9));
        assert!(!solver::is_valid_guess(board, 0, 0, 1));
        assert!(!solver::is_valid_guess(board, 0, 0, 7));
        assert!(!solver::is_valid_guess(board, 3, 5, 7));
        assert!(!solver::is_valid_guess(board, 3, 5, 4));
        assert!(!solver::is_valid_guess(board, 3, 5, 8));
    }

    #[test]
    fn valid_guesses() {
        let board = sudoku_board();
        assert!(solver::is_valid_guess(board, 0, 0, 2));
        assert!(solver::is_valid_guess(board, 1, 1, 5));
        assert!(solver::is_valid_guess(board, 0, 0, 2));
        assert!(solver::is_valid_guess(board, 4, 0, 3));
        assert!(solver::is_valid_guess(board, 4, 2, 1));
    }
}
