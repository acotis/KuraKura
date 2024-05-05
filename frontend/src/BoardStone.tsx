import { Stone } from "./types";
import "./BoardStone.css";

export interface BoardStoneProps {
  stone: Stone;
  preview?: boolean;
}

export function BoardStone({ stone, preview }: BoardStoneProps) {
  return (
    <div
      className={
        "stone stone-" + stone.color + (preview ? " stone-preview" : "")
      }
      style={{ transform: `rotate(${stone.rotation}deg)` }}
    >
      {stone.label}
    </div>
  );
}
