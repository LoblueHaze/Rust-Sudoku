/*
 *
 * Brute force sudoku solving algorithm using Rust.
 * By Loblue Haze <loblue.haze@gmail.com>
 * Reference: https://www.codesdope.com/blog/article/solving-sudoku-with-backtracking-c-java-and-python/
 * Basically, this is the rust version of the python code in the reference.
 * If you don't want animation, set DO_ANIMATION to false.
 *
 */

use std::{thread, time};

const SIZE: usize = 9;
const DO_ANIMATION: bool = true;

#[derive(Debug, Copy, Clone)]
struct Game {
    matrix: [[i32; 9]; 9],
}

impl Game {
    fn number_unassigned(self, row: &mut i32, col: &mut i32) -> [i32; 3] {
        let mut num_unassign: i32 = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.matrix[i][j] == 0 {
                    *row = i as i32;
                    *col = j as i32;
                    num_unassign = 1;
                    let a = [*row, *col, num_unassign];
                    return a;
                }
            }
        }
        let a = [-1, -1, num_unassign];
        a
    }

    fn is_safe(self, number: i32, row: i32, col: i32) -> bool {
        for i in 0..SIZE {
            if self.matrix[row as usize][i] == number {
                return false;
            }
        }
        for j in 0..SIZE {
            if self.matrix[j][col as usize] == number {
                return false;
            }
        }
        let row_start = row - (row % 3);
        let col_start = col - (col % 3);

        for i in row_start..row_start + 3 {
            for j in col_start..col_start + 3 {
                if self.matrix[i as usize][j as usize] == number {
                    return false;
                }
            }
        }
        true
    }

    fn print_sudoku(self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("{} ", self.matrix[i][j]);
                if j % 3 == 2 && j != SIZE - 1 {
                    print!("| ")
                }
            }
            println!("");
            if i % 3 == 2 && i != SIZE - 1 {
                println!("---------------------");
            }
        }
        println!("");
    }
}

#[derive(Debug, Clone, Copy)]
struct GameState {
    game: Game,
    is_solved: bool,
}

fn main() {
    let game = Game {
        matrix: [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ],
    };
    let game_state = GameState {
        game,
        is_solved: false,
    };
    let solved_state = solve_sudoku(game_state);
    if solved_state.is_solved {
        solved_state.game.print_sudoku();
    } else {
        println!("No solution!!");
    }
}

fn solve_sudoku(mut game_state: GameState) -> GameState {
    if DO_ANIMATION {
        game_state.game.print_sudoku();
        let sleep_time = time::Duration::from_millis(10);
        thread::sleep(sleep_time);
        print!("\x1B[2J\x1B[1;1H");
    }
    let mut row = 0;
    let mut col = 0;
    let a = game_state.game.number_unassigned(&mut row, &mut col);
    if a[2] == 0 {
        return GameState {
            game: game_state.game,
            is_solved: true,
        };
    }
    row = a[0];
    col = a[1];
    for i in 1..10 {
        if game_state.game.is_safe(i, row, col) {
            game_state.game.matrix[row as usize][col as usize] = i;
            let recur_sudoku = solve_sudoku(game_state);
            if recur_sudoku.is_solved {
                return recur_sudoku;
            }
            game_state.game.matrix[row as usize][col as usize] = 0;
        }
    }
    GameState {
        game: game_state.game,
        is_solved: false,
    }
}
