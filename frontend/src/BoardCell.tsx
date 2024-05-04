import { Color, boardSize } from "./types";
import "./BoardCell.css";

export interface BoardCellProps {
  x: number;
  y: number;
  color: Color | undefined;
  placePreview?: Color | undefined;
  onClick?: () => undefined;
}

export function BoardCell(props: BoardCellProps) {
  const clickable = props.placePreview && !props.color;
  const cellClass = clickable
    ? "board-cell board-cell-clickable"
    : "board-cell";
  return (
    <div className={cellClass} onClick={props.onClick}>
      {props.x > 0 && <div className="board-line board-line-l" />}
      {props.x < boardSize - 1 && <div className="board-line board-line-r" />}
      {props.y > 0 && <div className="board-line board-line-t" />}
      {props.y < boardSize - 1 && <div className="board-line board-line-b" />}
      {props.color ? (
        <div className={"stone stone-" + props.color} />
      ) : props.placePreview ? (
        <div className={"stone stone-preview stone-" + props.placePreview} />
      ) : undefined}
    </div>
  );
}
