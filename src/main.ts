import { invoke } from "@tauri-apps/api/tauri";

type CellState = "X" | "O" | "Empty";

type PlayState = "OTurn" | "XTurn" | "XWon" | "OWon" | "Draw";

type GameState = {
  state: PlayState;
  cells: Array<Array<CellState>>;
};

// Update game status
const updateGameStatus = (state: PlayState): void => {
  const statusText = document.getElementById("status");

  if (statusText) {
    // Format the status text for display
    switch (state) {
      case "OTurn":
        statusText.innerText = "O's Turn";
        break;
      case "XTurn":
        statusText.innerText = "X's Turn";
        break;
      case "XWon":
        statusText.innerText = "X Wins!";
        break;
      case "OWon":
        statusText.innerText = "O Wins!";
        break;
      case "Draw":
        statusText.innerText = "Draw!";
        break;
      default:
        statusText.innerText = "Unknown";
    }
  }
};

const drawBoard = (cells: Array<Array<CellState>>): void => {
  for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      const cell = document.getElementById(`cell-${i}-${j}`);
      if (cell) {
        switch (cells[i][j]) {
          case "X":
            cell.innerText = "X";
            break;
          case "O":
            cell.innerText = "O";
            break;
          default:
            cell.innerText = "";
        }
      }
    }
  }
};

const cellClicked = async (x: number, y: number): Promise<void> => {
  const result: GameState = JSON.parse(await invoke("click_cell", { x, y }));
  if (!result) {
    console.warn("No result");
    return;
  }

  drawBoard(result.cells);
  updateGameStatus(result.state);
};

const resetBoard = async (): Promise<void> => {
  const result = JSON.parse(await invoke("reset_board"));
  if (!result) {
    console.warn("No result");
    return;
  }

  drawBoard(result.cells);
  updateGameStatus(result.state);
};

window.addEventListener("DOMContentLoaded", () => {
  // Add click handlers to board cells
  for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      document
        .querySelector(`#cell-${i}-${j}`)
        ?.addEventListener("click", () => cellClicked(i, j));
    }
  }

  // Add click handler to reset button
  document
    .querySelector("#reset")
    ?.addEventListener("click", () => resetBoard());
});
