import { BoardLine, Cell, Color, Grid, Move } from "./types";
import update from "immutability-helper";

function rotateLine(line: BoardLine): BoardLine {
  return line === "top"
    ? "right"
    : line === "right"
    ? "bottom"
    : line === "bottom"
    ? "left"
    : "top";
}

function rotateCell(cell: Cell): Cell {
  return {
    stone: cell.stone
      ? { ...cell.stone, rotation: (cell.stone.rotation + 90) % 360 }
      : undefined,
    lines: cell.lines.map(rotateLine),
  };
}

export function applyMove(
  grid: Grid,
  move: Move,
  color: Color,
  label: string
): Grid {
  const newStone = { color, label, rotation: 0 };
  const placed = update(grid, {
    [move.placeY]: { [move.placeX]: { stone: { $set: newStone } } },
  });
  const sx = move.spinX;
  const sy = move.spinY;
  const n = move.spinSize;
  const spun = placed.map((row, y) =>
    row.map((cell, x) =>
      x >= sx && x < sx + n && y >= sy && y < sy + n
        ? rotateCell(placed[sy + n - 1 - (x - sx)][sx + (y - sy)])
        : cell
    )
  );
  return spun;
}
