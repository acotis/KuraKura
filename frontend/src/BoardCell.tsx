import { Cell, Stone } from "./types";
import "./BoardCell.css";
import { BoardStone } from "./BoardStone";

export interface BoardCellProps {
  cell: Cell;
  stonePreview?: Stone | undefined;
  onClick?: () => undefined;
}

export function BoardCell(props: BoardCellProps) {
  const { cell, stonePreview, onClick } = props;
  const clickable = stonePreview && !cell.stone;
  const cellClass = clickable
    ? "board-cell board-cell-clickable"
    : "board-cell";
  return (
    <div className={cellClass} onClick={onClick}>
      {cell.lines.map((x) => (
        <div key={x} className={"board-line board-line-" + x} />
      ))}
      {cell.stone ? (
        <BoardStone stone={cell.stone} />
      ) : stonePreview ? (
        <BoardStone preview stone={stonePreview} />
      ) : undefined}
    </div>
  );
}
