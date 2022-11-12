#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

struct BoardState(Arc<Mutex<Board>>);

// Manage possible states of tic-tac-toe cells
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum CellState {
    Empty,
    X,
    O,
}

// Manage possible states of the game
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum GameState {
    XTurn,
    OTurn,
    XWon,
    OWon,
    Draw,
}

// Manage the game board
#[derive(Clone, Copy, Serialize, Deserialize)]
struct Board {
    cells: [[CellState; 3]; 3],
    state: GameState,
}

// Initialize the game board
impl Board {
    fn new() -> Board {
        Board {
            cells: [[CellState::Empty; 3]; 3],
            state: GameState::XTurn,
        }
    }
}

// Manage the game logic
impl Board {
    fn reset(&mut self) {
        self.cells = [[CellState::Empty; 3]; 3];
        self.state = GameState::XTurn;
    }

    fn make_move(&mut self, row: usize, col: usize) {
        if self.state == GameState::XTurn {
            self.cells[row][col] = CellState::X;
            self.state = GameState::OTurn;
        } else if self.state == GameState::OTurn {
            self.cells[row][col] = CellState::O;
            self.state = GameState::XTurn;
        }
    }

    // Check if the game is a draw
    fn is_draw(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.cells[row][col] == CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn check_win(&mut self) {
        // Check rows
        for row in 0..3 {
            if self.cells[row][0] == CellState::X
                && self.cells[row][1] == CellState::X
                && self.cells[row][2] == CellState::X
            {
                self.state = GameState::XWon;
            }
            if self.cells[row][0] == CellState::O
                && self.cells[row][1] == CellState::O
                && self.cells[row][2] == CellState::O
            {
                self.state = GameState::OWon;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.cells[0][col] == CellState::X
                && self.cells[1][col] == CellState::X
                && self.cells[2][col] == CellState::X
            {
                self.state = GameState::XWon;
            }
            if self.cells[0][col] == CellState::O
                && self.cells[1][col] == CellState::O
                && self.cells[2][col] == CellState::O
            {
                self.state = GameState::OWon;
            }
        }

        // Check diagonals
        if self.cells[0][0] == CellState::X
            && self.cells[1][1] == CellState::X
            && self.cells[2][2] == CellState::X
        {
            self.state = GameState::XWon;
        }
        if self.cells[0][0] == CellState::O
            && self.cells[1][1] == CellState::O
            && self.cells[2][2] == CellState::O
        {
            self.state = GameState::OWon;
        }
        if self.cells[0][2] == CellState::X
            && self.cells[1][1] == CellState::X
            && self.cells[2][0] == CellState::X
        {
            self.state = GameState::XWon;
        }
        if self.cells[0][2] == CellState::O
            && self.cells[1][1] == CellState::O
            && self.cells[2][0] == CellState::O
        {
            self.state = GameState::OWon;
        }
    }
}

// Check the state of the clicked cell
#[tauri::command]
fn click_cell(x: i32, y: i32, board: State<'_, BoardState>) -> String {
    let mut board = board.0.lock().unwrap();
    let mut cell = board.cells[x as usize][y as usize];
    if cell == CellState::Empty {
        board.make_move(x as usize, y as usize);
        board.check_win();
    }

    serde_json::to_string(&board.clone()).unwrap()
}

// Reset the board
#[tauri::command]
fn reset_board(board: State<'_, BoardState>) -> String {
    let mut board = board.0.lock().unwrap();
    board.reset();
    serde_json::to_string(&board.clone()).unwrap()
}

fn main() {
    tauri::Builder::default()
        .manage(BoardState(Arc::new(Mutex::new(Board::new()))))
        .invoke_handler(tauri::generate_handler![click_cell, reset_board])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
