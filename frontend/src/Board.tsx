import { Color, Move } from "./types";
import "./Board.css";
import { useState } from "react";

export interface BoardCellProps {
  x: number;
  y: number;
  color: Color | undefined;
}

export function BoardCell(props: BoardCellProps) {
  return (
    <div className="board-cell">
      {props.x > 0 && <div className="board-line board-line-l" />}
      {props.x < 5 && <div className="board-line board-line-r" />}
      {props.y > 0 && <div className="board-line board-line-t" />}
      {props.y < 5 && <div className="board-line board-line-b" />}
      {props.color ? (
        <div className={"stone stone-" + props.color} />
      ) : undefined}
    </div>
  );
}

export interface BoardProps {
  grid: (Color | undefined)[][];
  /**
   * A Color means the board is interactable as that player. `undefined` means
   * the game is not yet started, or already over, or we're waiting for the
   * other player's turn.
   */
  active: Color | undefined;

  /**
   * Called when the player locks in their move.
   */
  onMove: (move: Move) => undefined;
}

export function Board(props: BoardProps) {
  return (
    <div>
      <table className="board-table">
        <tbody>
          {props.grid.map((row, y) => (
            <tr className="board-tr" key={y}>
              {row.map((color, x) => (
                <td className="board-td">
                  <BoardCell y={y} x={x} color={color} key={x} />
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
