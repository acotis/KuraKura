// import { useState } from "react";
import { useState } from "react";
import Board from "./Board";
import { Color, Grid, boardLinesFor, boardSize } from "./types";
import { applyMove } from "./logic";

function Game() {
  const params = new URLSearchParams(location.search);
  const gameId = params.get("id");
  console.log(gameId);

  const [grid, setGrid] = useState<Grid>(
    new Array(boardSize).fill(undefined).map((_, y) =>
      new Array(boardSize).fill(undefined).map((_, x) => ({
        stone: undefined,
        lines: boardLinesFor(x, y, boardSize),
      }))
    )
  );
  const [active, setActive] = useState<Color>("black");
  const [moveNumber, setMoveNumber] = useState(1);

  return (
    <Board
      grid={grid}
      tileSize={64}
      active={active}
      moveNumber={moveNumber}
      onMove={(move) => {
        setGrid(applyMove(grid, move, active, moveNumber.toString()));
        setActive(active === "black" ? "white" : "black");
        setMoveNumber(moveNumber + 1);
      }}
    />
  );
}

export default Game;
