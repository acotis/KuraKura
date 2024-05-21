export const boardSize = 6;

export type Color = "black" | "white";

export type Stone = {
  color: Color;
  label: string;
  /**
   * In degrees.
   */
  rotation: number;
};

export type BoardLine = "top" | "right" | "bottom" | "left";

export type Cell = {
  stone: Stone | undefined;
  /**
   * Which lines does this cell have?
   * "top" means a line from the center to the top,
   * "right" means a line from the center to the right edge, etc.
   */
  lines: Array<BoardLine>;
};

export function boardLinesFor(
  x: number,
  y: number,
  boardSize: number
): BoardLine[] {
  const lines: BoardLine[] = [];
  if (y > 0) lines.push("top");
  if (x < boardSize - 1) lines.push("right");
  if (y < boardSize - 1) lines.push("bottom");
  if (x > 0) lines.push("left");
  return lines;
}

export type Grid = Cell[][];

export interface Move {
  placeX: number;
  placeY: number;
  spinX: number;
  spinY: number;
  spinSize: number;
}

export type MoveState =
  | { phase: "place" }
  | { phase: "spin"; move: Pick<Move, "placeX" | "placeY"> };

export type SpinState =
  | { phase: "start" }
  | { phase: "drag"; x1: number; y1: number }
  | { phase: "preview" }
  | { phase: "cancel" };
