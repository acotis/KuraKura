import { Cell, Stone } from "./types";
import "./BoardCell.css";

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
        <div
          className={"stone stone-" + cell.stone.color}
          style={{ transform: `rotate(${cell.stone.rotation}deg)` }}
        >
          {cell.stone.label}
        </div>
      ) : stonePreview ? (
        <div className={"stone stone-preview stone-" + stonePreview.color} />
      ) : undefined}
    </div>
  );
}
