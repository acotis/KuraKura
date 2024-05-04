import { Color, Move, MoveState, SpinState, tileSize } from "./types";
import "./Board.css";
import { useCallback, useState } from "react";
import { BoardCell } from "./BoardCell";
import { SpinPreview } from "./SpinPreview";
import update from "immutability-helper";

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

type SpinRect = {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
};

export function Board(props: BoardProps) {
  const [moveState, setMoveState] = useState<MoveState>({ phase: "place" });
  const [spin, setSpin] = useState<SpinState>({ phase: "start" });
  const [spinRect, setSpinRect] = useState<SpinRect | undefined>(undefined);

  const spinMouseMove = useCallback(
    (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
      const rect = (e.target! as HTMLDivElement).getBoundingClientRect();
      const x = Math.floor((e.clientX - rect.left) / tileSize);
      const y = Math.floor((e.clientY - rect.top) / tileSize);
      if (spin.phase === "drag") {
        setSpinRect({ x1: spin.x1, y1: spin.y1, x2: x, y2: y });
      } else if (spin.phase === "start") {
        setSpinRect({ x1: x, y1: y, x2: x, y2: y });
      }
    },
    [spin]
  );

  const spinMouseDown = useCallback(
    (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
      if (spin.phase === "preview") {
        setSpinRect(undefined);
        setSpin({ phase: "cancel" });
        return;
      }
      const rect = (e.target! as HTMLDivElement).getBoundingClientRect();
      const x1 = Math.floor((e.clientX - rect.left) / tileSize);
      const y1 = Math.floor((e.clientY - rect.top) / tileSize);
      setSpin({ phase: "drag", x1, y1 });
    },
    [spin]
  );

  const spinMouseUp = useCallback(() => {
    if (spin.phase === "cancel") {
      setSpin({ phase: "start" });
    } else if (spin.phase === "drag" && spinRect) {
      const sw = Math.abs(spinRect.x1 - spinRect.x2) + 1;
      const sh = Math.abs(spinRect.y1 - spinRect.y2) + 1;
      if (sw === sh) {
        setSpin({ phase: "preview" });
      } else {
        setSpin({ phase: "start" });
      }
    }
  }, [spin]);

  const newGrid =
    moveState.phase === "place"
      ? props.grid
      : update(props.grid, {
          [moveState.move.placeY]: {
            [moveState.move.placeX]: { $set: props.active },
          },
        });

  return (
    <div className="flexv">
      <div className="board">
        <table
          className="board-table"
          style={{ opacity: spin.phase === "preview" ? 0.7 : 1 }}
        >
          <tbody>
            {newGrid.map((row, y) => (
              <tr className="board-tr" key={y}>
                {row.map((color, x) => (
                  <td key={x} className="board-td">
                    {spinRect &&
                    spin.phase === "preview" &&
                    x >= Math.min(spinRect.x1, spinRect.x2) &&
                    x <= Math.max(spinRect.x1, spinRect.x2) &&
                    y >= Math.min(spinRect.y1, spinRect.y2) &&
                    y <= Math.max(spinRect.y1, spinRect.y2) ? undefined : (
                      <BoardCell
                        y={y}
                        x={x}
                        color={color}
                        placePreview={
                          moveState.phase === "place" ? props.active : undefined
                        }
                        onClick={() => {
                          if (
                            moveState.phase === "place" &&
                            props.grid[y][x] === undefined
                          ) {
                            setMoveState({
                              phase: "spin",
                              move: { placeX: x, placeY: y },
                            });
                          }
                        }}
                      />
                    )}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
        {moveState.phase === "spin" && (
          <div
            className="spin-area"
            onMouseMove={spinMouseMove}
            onMouseDown={spinMouseDown}
            onMouseUp={spinMouseUp}
          ></div>
        )}
        {moveState.phase === "spin" && spinRect && (
          <SpinPreview grid={newGrid} spinRect={spinRect} spin={spin} />
        )}
      </div>
      <button
        disabled={moveState.phase !== "spin" || spin.phase !== "preview"}
        onClick={() => {
          if (moveState.phase === "spin" && spinRect) {
            setMoveState({ phase: "place" });
            setSpin({ phase: "start" });
            setSpinRect(undefined);
            props.onMove({
              placeX: moveState.move.placeX,
              placeY: moveState.move.placeY,
              spinX: Math.min(spinRect.x1, spinRect.x2),
              spinY: Math.min(spinRect.y1, spinRect.y2),
              spinSize: Math.abs(spinRect.x1 - spinRect.x2) + 1,
            });
          }
        }}
      >
        Confirm
      </button>
    </div>
  );
}