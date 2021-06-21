pub mod solver {
    pub type SudokuBoard = [[i8; 9]; 9];

    // is_valid_guess: given a board, row, column, and guess, returns true if
    //                 guess at specified index does not invalidate board.
    pub fn is_valid_guess(board: SudokuBoard, row: usize, col: usize, guess: i8) -> bool {
        // Case: row or col out of bounds.
        if row > 8 || col > 8 {
            return false;
        }

        // Case: guess at [row, col] would make row or col invalid.
        for i in 0..9 {
            if board[i][col] == guess || board[row][i] == guess {
                return false;
            }
        }

        // Case: guess would cause the current block to be invalid.
        let block_x: usize = row / 3 * 3;
        let block_y: usize = col / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if board[block_x + x][block_y + y] == guess {
                    return false;
                }
            }
        }

        true
    }

    // get_next_empty: returns the row, col of the next empty cell in the board
    //                 as well as the solved state of the board.
    pub fn get_next_empty(board: SudokuBoard) -> (usize, usize, bool) {
        for x in 0..9 {
            for y in 0..9 {
                if board[x][y] == 0 {
                    return (x, y, false);
                }
            }
        }

        (42, 42, true)
    }

    // is_solved: returns the solved state of the board
    pub fn is_solved(board: SudokuBoard) -> bool {
        let (_, _, solved) = get_next_empty(board);

        solved
    }

    // find_all_guesses: returns all the valid guesses of the board at the
    //                   given row, col
    pub fn find_all_guesses(board: SudokuBoard, row: usize, col: usize) -> Vec<i8> {
        let mut guesses: Vec<i8> = vec![];
        for guess in 1..10 {
            if is_valid_guess(board, row, col, guess) {
                guesses.push(guess);
            }
        }

        guesses
    }

    // solve: solves the sudoku puzzle
    pub fn solve(board: SudokuBoard) -> SudokuBoard {
        let mut result: SudokuBoard = board;

        let (row, col, solved): (usize, usize, bool) = get_next_empty(board);

        if solved {
            return result;
        }

        for guess in find_all_guesses(board, row, col) {
            result[row][col] = guess;
            result = solve(result);
            if is_solved(result) {
                return result;
            }
        }

        result = board;
        result
    }
}
