import { Color, Grid, Move, MoveState, SpinState, Stone } from "./types";
import { useCallback, useState } from "react";
import BoardCell from "./BoardCell";
import SpinPreview from "./SpinPreview";
import update from "immutability-helper";

export interface BoardProps {
  tileSize: number;
  grid: Grid;
  /**
   * A Color means the board is interactable as that player. `undefined` means
   * the game is not yet started, or already over, or we're waiting for the
   * other player's turn.
   */
  active: Color | undefined;
  moveNumber: number;
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

export default function Board(props: BoardProps) {
  const [moveState, setMoveState] = useState<MoveState>({ phase: "place" });
  const [spin, setSpin] = useState<SpinState>({ phase: "start" });
  const [spinRect, setSpinRect] = useState<SpinRect | undefined>(undefined);
  const { tileSize } = props;

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
    [spin, tileSize]
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
    [spin, tileSize]
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
  }, [spinRect, spin]);

  let newGrid = props.grid;
  const newStone: Stone | undefined = props.active
    ? { color: props.active, label: props.moveNumber.toString(), rotation: 0 }
    : undefined;

  if (props.active && moveState.phase !== "place") {
    newGrid = update(props.grid, {
      [moveState.move.placeY]: {
        [moveState.move.placeX]: {
          stone: { $set: newStone },
        },
      },
    });
  }

  return (
    <div className="fcc gap-4">
      <div className="shadow-lg p-[8px] bg-board rounded-lg">
        <div className="relative bg-base-100">
          <table style={{ opacity: spin.phase === "preview" ? 0.7 : 1 }}>
            <tbody>
              {newGrid.map((row, y) => (
                <tr key={y}>
                  {row.map((cell, x) => (
                    <td
                      key={x}
                      className="relative p-0 m-0"
                      style={{ width: 40, height: 40 }}
                    >
                      {spinRect &&
                      spin.phase === "preview" &&
                      x >= Math.min(spinRect.x1, spinRect.x2) &&
                      x <= Math.max(spinRect.x1, spinRect.x2) &&
                      y >= Math.min(spinRect.y1, spinRect.y2) &&
                      y <= Math.max(spinRect.y1, spinRect.y2) ? undefined : (
                        <BoardCell
                          cell={cell}
                          stonePreview={newStone}
                          onClick={() => {
                            if (
                              moveState.phase === "place" &&
                              props.grid[y][x].stone === undefined
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
          {props.active && moveState.phase === "spin" && (
            <div
              className="z-9 inset-0 cursor-pointer absolute"
              onMouseMove={spinMouseMove}
              onMouseDown={spinMouseDown}
              onMouseUp={spinMouseUp}
            ></div>
          )}
          {props.active && moveState.phase === "spin" && spinRect && (
            <SpinPreview
              grid={newGrid}
              tileSize={tileSize}
              spinRect={spinRect}
              spin={spin}
            />
          )}
        </div>
      </div>
      {props.active && (
        <button
          className="btn no-animation"
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
      )}
    </div>
  );
}
