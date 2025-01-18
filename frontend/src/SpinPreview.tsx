import BoardCell from "./BoardCell";
import type { Grid, SpinState } from "./types";

export interface SpinPreviewProps {
	grid: Grid;
	tileSize: number;
	spinRect: {
		x1: number;
		y1: number;
		x2: number;
		y2: number;
	};
	spin: SpinState;
}

export default function SpinPreview(props: SpinPreviewProps) {
	const { spinRect, tileSize, spin } = props;
	const left = Math.max(Math.min(spinRect.x1, spinRect.x2), 0);
	const top = Math.max(Math.min(spinRect.y1, spinRect.y2), 0);
	const width = Math.abs(spinRect.x1 - spinRect.x2) + 1;
	const height = Math.abs(spinRect.y1 - spinRect.y2) + 1;
	const rectClass =
		width !== height
			? " outline outline-error bg-white/30"
			: spin.phase === "preview"
				? ""
				: " outline outline-info bg-white/30";

	return (
		<>
			<div
				className={`pointer-events-none absolute${rectClass}`}
				style={{
					left: left * tileSize,
					top: top * tileSize,
					width: width * tileSize,
					height: height * tileSize,
				}}
			/>
			{spin.phase === "preview" && (
				<div
					className={`absolute ${width === height ? "animate-cw" : ""}`}
					style={{ left: left * tileSize, top: top * tileSize }}
				>
					<div className="absolute inset-0 z-10 pointer-events-none w-full h-full bg-white/30" />
					<table className={"outline outline-info"}>
						<tbody>
							{props.grid.slice(top, top + width).map((row, y) => (
								// biome-ignore lint/suspicious/noArrayIndexKey: Board coordinate
								<tr key={y}>
									{row.slice(left, left + width).map((cell, x) => (
										<td
											// biome-ignore lint/suspicious/noArrayIndexKey: Board coordinate
											key={x}
											className="relative p-0 m-0"
											style={{ width: tileSize, height: tileSize }}
										>
											<BoardCell cell={cell} />
										</td>
									))}
								</tr>
							))}
						</tbody>
					</table>
				</div>
			)}
		</>
	);
}
