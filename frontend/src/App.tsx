// import { useState } from "react";
import { useState } from "react";
import { Board } from "./Board";
import { BoardLine, Cell, Color, Grid, Move, boardSize } from "./types";
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

function applyMove(grid: Grid, move: Move, color: Color, label: string): Grid {
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

function linesFor(x: number, y: number, boardSize: number): BoardLine[] {
  const lines: BoardLine[] = [];
  if (y > 0) lines.push("top");
  if (x < boardSize - 1) lines.push("right");
  if (y < boardSize - 1) lines.push("bottom");
  if (x > 0) lines.push("left");
  return lines;
}

function App() {
  const [grid, setGrid] = useState<Grid>(
    new Array(boardSize).fill(undefined).map((_, y) =>
      new Array(boardSize).fill(undefined).map((_, x) => ({
        stone: undefined,
        lines: linesFor(x, y, boardSize),
      }))
    )
  );
  const [active, setActive] = useState<Color>("black");
  const [moveNumber, setMoveNumber] = useState(1);

  return (
    <div className="text-center">
      <h1>kurakura!</h1>
      <Board
        grid={grid}
        active={active}
        moveNumber={moveNumber}
        onMove={(move) => {
          setGrid(applyMove(grid, move, active, moveNumber.toString()));
          setActive(active === "black" ? "white" : "black");
          setMoveNumber(moveNumber + 1);
        }}
      />
    </div>
  );
}

export default App;
