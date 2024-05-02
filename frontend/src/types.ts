export type Color = "black" | "white";

export interface Move {
  placeX: number;
  placeY: number;
  spinX: number;
  spinY: number;
  spinSize: number;
}
