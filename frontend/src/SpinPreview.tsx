import { BoardCell } from "./BoardCell";
import { Color, SpinState, tileSize } from "./types";

export interface SpinPreviewProps {
  grid: (Color | undefined)[][];
  spinRect: {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
  };
  spin: SpinState;
}

export function SpinPreview(props: SpinPreviewProps) {
  const { spinRect, spin } = props;
  const left = Math.min(spinRect.x1, spinRect.x2);
  const top = Math.min(spinRect.y1, spinRect.y2);
  const width = Math.abs(spinRect.x1 - spinRect.x2) + 1;
  const height = Math.abs(spinRect.y1 - spinRect.y2) + 1;
  const rectClass =
    width !== height
      ? " spin-rect-bad"
      : spin.phase === "preview"
      ? " spin-rect-ok"
      : "";

  return (
    <>
      <div
        className={"spin-rect" + rectClass}
        style={{
          left: left * tileSize,
          top: top * tileSize,
          width: width * tileSize,
          height: height * tileSize,
        }}
      />
      {spin.phase === "preview" && (
        <table
          className={
            "board-table spin-preview" +
            (width === height ? " spin-preview-animate" : "")
          }
          style={{ left: left * tileSize, top: top * tileSize }}
        >
          <tbody>
            {props.grid.slice(top, top + width).map((row, y) => (
              <tr className="board-tr" key={y}>
                {row.slice(left, left + width).map((color, x) => (
                  <td key={x} className="board-td">
                    <BoardCell y={top + y} x={left + x} color={color} />
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </>
  );
}
