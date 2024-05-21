import BoardCell from "./BoardCell";
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

export default function SpinPreview(props: SpinPreviewProps) {
  const { spinRect, spin } = props;
  const left = Math.max(Math.min(spinRect.x1, spinRect.x2), 0);
  const top = Math.max(Math.min(spinRect.y1, spinRect.y2), 0);
  const width = Math.abs(spinRect.x1 - spinRect.x2) + 1;
  const height = Math.abs(spinRect.y1 - spinRect.y2) + 1;
  const rectClass =
    width !== height
      ? " outline-dashed outline-error bg-error/20"
      : spin.phase === "preview"
      ? ""
      : " outline-dashed outline-info bg-info/20";

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
            "outline-dashed outline-info absolute" +
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
