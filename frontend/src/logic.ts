import {
	type BoardLine,
	type Cell,
	type Color,
	type Grid,
	type Move,
} from "./types";
import update from "immutability-helper";

function rotateLine(line: BoardLine): BoardLine {
	return line === "top"
		? "right"
		: line === "right"
			? "bottom"
			: line === "bottom"
				? "left"
				: "top";
}

function rotateCell(cell: Cell): Cell {
	return {
		stone: cell.stone
			? { ...cell.stone, rotation: (cell.stone.rotation + 90) % 360 }
			: undefined,
	};
}

export function applySpin(grid: Grid, sx: number, sy: number, n: number): Grid {
	return grid.map((row, y) =>
		row.map((cell, x) =>
			x >= sx && x < sx + n && y >= sy && y < sy + n
				? rotateCell(grid[sy + n - 1 - (x - sx)][sx + (y - sy)])
				: cell,
		),
	);
}

export function applyMove(
	grid: Grid,
	move: Move,
	color: Color,
	label: string,
): Grid {
	const newStone = { color, label, rotation: 0 };
	const placed = update(grid, {
		[move.placeY]: { [move.placeX]: { stone: { $set: newStone } } },
	});
	return applySpin(placed, move.spinX, move.spinY, move.spinSize);
}
