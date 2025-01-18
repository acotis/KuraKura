import { Stone } from "./types";

export interface BoardStoneProps {
  stone: Stone;
  preview?: boolean;
}

export default function BoardStone({ stone, preview }: BoardStoneProps) {
  let className =
    "pointer-events-none absolute rounded-full w-full h-full flex items-center justify-center text-2xl";
  if (stone.color === "Black") className += " bg-gray-700 text-gray-200";
  if (stone.color === "White") className += " bg-gray-200 text-gray-700";
  if (preview) className += " opacity-0 group-hover:opacity-50";
  return (
    <div
      className={className}
      style={{ transform: `rotate(${stone.rotation}deg)` }}
    >
      {stone.label}
    </div>
  );
}
