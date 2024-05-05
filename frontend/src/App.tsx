// import { useState } from "react";
import { useState } from "react";
import "./App.css";
import { Board } from "./Board";
import { BoardLine, Cell, Color, Grid, Move, boardSize } from "./types";
import update from "immutability-helper";

function rotateLine(line: BoardLine): BoardLine {
  return line === "t" ? "r" : line === "r" ? "b" : line === "b" ? "l" : "t";
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

function App() {
  const [grid, setGrid] = useState<Grid>(
    new Array(boardSize).fill(undefined).map((_, y) =>
      new Array(boardSize).fill(undefined).map((_, x) => ({
        stone: undefined,
        lines: (y > 0 ? ["t" as BoardLine] : [])
          .concat(x < boardSize - 1 ? ["r" as BoardLine] : [])
          .concat(y < boardSize - 1 ? ["b" as BoardLine] : [])
          .concat(x > 0 ? ["l" as BoardLine] : []),
      }))
    )
  );
  const [active, setActive] = useState<Color>("black");
  const [moveNumber, setMoveNumber] = useState(1);

  return (
    <>
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
    </>
  );
}

export default App;
