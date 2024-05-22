import { BoardLine, Cell, Stone } from "./types";
import BoardStone from "./BoardStone";

export interface BoardCellProps {
  cell: Cell;
  stonePreview?: Stone | undefined;
  onClick?: () => undefined;
}

function Line({ line }: { line: BoardLine }) {
  let classes = "absolute outline-black outline outline-1 inset-1/2";
  switch (line) {
    case "top":
      classes += " top-0";
      break;
    case "right":
      classes += " right-0";
      break;
    case "bottom":
      classes += " bottom-0";
      break;
    case "left":
      classes += " left-0";
      break;
  }
  return <div className={classes} />;
}

export default function BoardCell(props: BoardCellProps) {
  const { cell, stonePreview, onClick } = props;
  const clickable = stonePreview && !cell.stone;
  let cellClass = "bg-board w-full h-full flex";
  if (clickable) cellClass += " group cursor-pointer";
  return (
    <div className={cellClass} onClick={onClick}>
      {cell.lines.map((x) => (
        <Line key={x} line={x} />
      ))}
      {cell.stone ? (
        <BoardStone stone={cell.stone} />
      ) : stonePreview ? (
        <BoardStone preview stone={stonePreview} />
      ) : undefined}
    </div>
  );
}
