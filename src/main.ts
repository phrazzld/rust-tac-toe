import { invoke } from "@tauri-apps/api/tauri";

const cellClicked = async (x: number, y: number): Promise<void> => {
  const result = await invoke("click_cell", { x, y });
  console.log(result);
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
});
