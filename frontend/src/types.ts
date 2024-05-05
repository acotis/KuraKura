export const boardSize = 6;
export const tileSize = 64;

export type Color = "black" | "white";

export type Stone = {
  color: Color;
  label: string;
  /**
   * In degrees.
   */
  rotation: number;
};

export type BoardLine = "t" | "r" | "b" | "l";

export type Cell = {
  stone: Stone | undefined;
  /**
   * Which lines does this cell have?
   * "t" means a line from the center to the top,
   * "r" means a line from the center to the right edge, etc.
   */
  lines: Array<BoardLine>;
};

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
