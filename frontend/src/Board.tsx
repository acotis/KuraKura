import { Color, Move } from "./types";
import "./Board.css";
import { useState } from "react";

const tileSize = 64;

export interface BoardCellProps {
  x: number;
  y: number;
  color: Color | undefined;
  placePreview: Color | undefined;
  onClick: () => undefined;
}

export function BoardCell(props: BoardCellProps) {
  const clickable = props.placePreview && !props.color;
  const cellClass = clickable
    ? "board-cell board-cell-clickable"
    : "board-cell";
  return (
    <div className={cellClass} onClick={props.onClick}>
      {props.x > 0 && <div className="board-line board-line-l" />}
      {props.x < 5 && <div className="board-line board-line-r" />}
      {props.y > 0 && <div className="board-line board-line-t" />}
      {props.y < 5 && <div className="board-line board-line-b" />}
      {props.color ? (
        <div className={"stone stone-" + props.color} />
      ) : props.placePreview ? (
        <div className={"stone stone-preview stone-" + props.placePreview} />
      ) : undefined}
    </div>
  );
}

export interface SpinCellProps {
  x: number;
  y: number;
  color: Color | undefined;
  onClick: () => undefined;
}

export function SpinCell(props: SpinCellProps) {
  return <div className="spin-cell" onClick={props.onClick}></div>;
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

type MoveState =
  | { phase: "place" }
  | { phase: "spin"; move: Pick<Move, "placeX" | "placeY"> }
  | { phase: "confirm"; move: Move };

type Spin =
  | { phase: "start" }
  | { phase: "drag"; x1: number; y1: number }
  | { phase: "preview" }
  | { phase: "cancel" };

type SpinRect = {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
};

export function Board(props: BoardProps) {
  const [moveState, setMoveState] = useState<MoveState>({ phase: "place" });
  const [spin, setSpin] = useState<Spin>({ phase: "start" });
  const [spinRect, setSpinRect] = useState<SpinRect | undefined>(undefined);

  function cell(x: number, y: number, color: Color | undefined) {
    const placedColor =
      moveState.phase === "place"
        ? color
        : x === moveState.move.placeX && y === moveState.move.placeY
        ? props.active
        : color;

    return (
      <BoardCell
        y={y}
        x={x}
        color={placedColor}
        placePreview={moveState.phase === "place" ? props.active : undefined}
        onClick={() => {
          if (moveState.phase === "place") {
            setMoveState({
              phase: "spin",
              move: { placeX: x, placeY: y },
            });
          }
        }}
      />
    );
  }

  function spinMouseMove(e: React.MouseEvent<HTMLDivElement, MouseEvent>) {
    const rect = (e.target! as HTMLDivElement).getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) / tileSize);
    const y = Math.floor((e.clientY - rect.top) / tileSize);
    if (spin.phase === "drag") {
      setSpinRect({ x1: spin.x1, y1: spin.y1, x2: x, y2: y });
    } else if (spin.phase === "start") {
      setSpinRect({ x1: x, y1: y, x2: x, y2: y });
    }
  }

  function spinMouseDown(e: React.MouseEvent<HTMLDivElement, MouseEvent>) {
    if (spin.phase === "preview") {
      setSpinRect(undefined);
      setSpin({ phase: "cancel" });
      return;
    }
    const rect = (e.target! as HTMLDivElement).getBoundingClientRect();
    const x1 = Math.floor((e.clientX - rect.left) / tileSize);
    const y1 = Math.floor((e.clientY - rect.top) / tileSize);
    setSpin({ phase: "drag", x1, y1 });
  }

  function spinMouseUp() {
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
  }

  return (
    <div className="flexv">
      <div className="board">
        <table
          className="board-table"
          style={{ opacity: spin.phase === "preview" ? 0.7 : 1 }}
        >
          <tbody>
            {props.grid.map((row, y) => (
              <tr className="board-tr" key={y}>
                {row.map((color, x) => (
                  <td key={x} className="board-td">
                    {spinRect &&
                    spin.phase === "preview" &&
                    x >= Math.min(spinRect.x1, spinRect.x2) &&
                    x <= Math.max(spinRect.x1, spinRect.x2) &&
                    y >= Math.min(spinRect.y1, spinRect.y2) &&
                    y <= Math.max(spinRect.y1, spinRect.y2)
                      ? undefined
                      : cell(x, y, color)}
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
        {moveState.phase === "spin" &&
          spinRect &&
          (() => {
            const sx = Math.min(spinRect.x1, spinRect.x2);
            const sy = Math.min(spinRect.y1, spinRect.y2);
            const sw = Math.abs(spinRect.x1 - spinRect.x2) + 1;
            const sh = Math.abs(spinRect.y1 - spinRect.y2) + 1;

            return (
              <>
                <div
                  className={
                    "spin-rect" +
                    (sw !== sh
                      ? " spin-rect-bad"
                      : spin.phase === "preview"
                      ? " spin-rect-ok"
                      : "")
                  }
                  style={{
                    left: sx * tileSize,
                    top: sy * tileSize,
                    width: sw * tileSize,
                    height: sh * tileSize,
                  }}
                />
                {spin.phase === "preview" && (
                  <table
                    className={
                      "board-table spin-preview" +
                      (sw === sh ? " spin-preview-animate" : "")
                    }
                    style={{
                      left: sx * tileSize,
                      top: sy * tileSize,
                    }}
                  >
                    <tbody>
                      {props.grid.map((row, y) =>
                        y >= sy && y <= Math.max(spinRect.y1, spinRect.y2) ? (
                          <tr className="board-tr" key={y}>
                            {row.map((color, x) =>
                              x >= sx &&
                              x <= Math.max(spinRect.x1, spinRect.x2) ? (
                                <td key={x} className="board-td">
                                  {cell(x, y, color)}
                                </td>
                              ) : undefined
                            )}
                          </tr>
                        ) : undefined
                      )}
                    </tbody>
                  </table>
                )}
              </>
            );
          })()}
      </div>
    </div>
  );
}
