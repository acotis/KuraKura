import { BoardCell } from "./BoardCell";
import { Grid, SpinState, tileSize } from "./types";

export interface SpinPreviewProps {
  grid: Grid;
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
      ? " outline-dashed outline-red-500 bg-red-500/20"
      : spin.phase === "preview"
      ? ""
      : " outline-dashed outline-blue-500 bg-blue-500/20";

  return (
    <>
      <div
        className={"pointer-events-none absolute" + rectClass}
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
            "outline-dashed outline-blue-500 absolute" +
            (width === height ? " animate-cw" : "")
          }
          style={{ left: left * tileSize, top: top * tileSize }}
        >
          <tbody>
            {props.grid.slice(top, top + width).map((row, y) => (
              <tr key={y}>
                {row.slice(left, left + width).map((cell, x) => (
                  <td key={x} className="relative p-0 m-0 w-[64px] h-[64px]">
                    <BoardCell cell={cell} />
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
