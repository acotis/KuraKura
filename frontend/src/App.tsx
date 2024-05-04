// import { useState } from "react";
import { useState } from "react";
import "./App.css";
import { Board } from "./Board";
import { Color, Move } from "./types";
import update from "immutability-helper";

type Grid = (Color | undefined)[][];

function applyMove(grid: Grid, move: Move, color: Color): Grid {
  const placed = update(grid, {
    [move.placeY]: { [move.placeX]: { $set: color } },
  });
  const sx = move.spinX;
  const sy = move.spinY;
  const n = move.spinSize;
  const spun = placed.map((row, y) =>
    row.map((cell, x) =>
      x >= sx && x < sx + n && y >= sy && y < sy + n
        ? placed[sy + n - 1 - (x - sx)][sx + (y - sy)]
        : cell
    )
  );
  return spun;
}

function App() {
  const [grid, setGrid] = useState<Grid>([
    [undefined, undefined, undefined, undefined, undefined, undefined],
    [undefined, undefined, undefined, undefined, undefined, undefined],
    [undefined, undefined, "white", undefined, undefined, undefined],
    [undefined, undefined, undefined, "black", undefined, undefined],
    [undefined, undefined, undefined, undefined, undefined, undefined],
    [undefined, undefined, undefined, undefined, undefined, undefined],
  ]);
  const [active, setActive] = useState<Color>("black");

  return (
    <>
      <h1>kurakura!</h1>
      <Board
        grid={grid}
        active={active}
        onMove={(move) => {
          setGrid(applyMove(grid, move, active));
          setActive(active === "black" ? "white" : "black");
        }}
      />
    </>
  );
}

export default App;
