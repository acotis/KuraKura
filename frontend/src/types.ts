export const boardSize = 6;
export const tileSize = 64;

export type Color = "black" | "white";

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
