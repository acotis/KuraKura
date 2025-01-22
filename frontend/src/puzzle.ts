import { applyMove, applySpin } from "./logic";
import type { Color, Grid } from "./types";

const boardSize = 6;
const winSize = 5;

function winner(grid: Grid, n = 5): Color | undefined {
	for (const c of ["Black", "White"] as Color[]) {
		for (let y = 0; y < grid.length; y++) {
			for (let x = 0; x <= grid[y].length - n; x++) {
				if (
					[...Array(n).keys()].every((i) => grid[y][x + i].stone?.color === c)
				)
					return c;
			}
		}
		for (let x = 0; x < grid[0].length; x++) {
			for (let y = 0; y <= grid.length - n; y++) {
				if (
					[...Array(n).keys()].every((i) => grid[y + i][x].stone?.color === c)
				)
					return c;
			}
		}
		for (let x = 0; x <= grid[0].length - n; x++) {
			for (let y = 0; y <= grid.length - n; y++) {
				if (
					[...Array(n).keys()].every(
						(i) => grid[y + i][x + i].stone?.color === c,
					) ||
					[...Array(n).keys()].every(
						(i) => grid[y + i][x + n - 1 - i].stone?.color === c,
					)
				)
					return c;
			}
		}
	}
}

function emptyGrid(size: number): Grid {
	return new Array(size).fill(undefined).map((_) =>
		new Array(size).fill(undefined).map((_) => ({
			stone: undefined,
		})),
	);
}

function randomHorizontalWin(): Grid {
	const y = (Math.random() * boardSize) | 0;
	const x = (Math.random() * (boardSize - winSize + 1)) | 0;
	const grid = emptyGrid(boardSize);
	for (let i = 0; i < winSize; i++)
		grid[y][x + i].stone = { color: "Black", label: "", rotation: 0 };
	return grid;
}

function randomVerticalWin(): Grid {
	return applySpin(randomHorizontalWin(), 0, 0, boardSize);
}

function randomBackslashWin(): Grid {
	const y = (Math.random() * (boardSize - winSize + 1)) | 0;
	const x = (Math.random() * (boardSize - winSize + 1)) | 0;
	const grid = emptyGrid(boardSize);
	for (let i = 0; i < winSize; i++)
		grid[y + i][x + i].stone = { color: "Black", label: "", rotation: 0 };
	return grid;
}

function randomSlashWin(): Grid {
	return applySpin(randomBackslashWin(), 0, 0, boardSize);
}

function randomOrthogonalWin(): Grid {
	return Math.random() < 0.5 ? randomHorizontalWin() : randomVerticalWin();
}

function randomDiagonalWin(): Grid {
	return Math.random() < 0.5 ? randomSlashWin() : randomBackslashWin();
}

function randomWin(): Grid {
	return Math.random() < 0.5 ? randomOrthogonalWin() : randomDiagonalWin();
}

function unspin(grid: Grid, sx: number, sy: number, n: number): Grid {
	let spun = grid;
	for (let i = 0; i < 3; i++) spun = applySpin(spun, sx, sy, n);
	return spun;
}

function solutions(grid: Grid): number {
	let sols = 0;
	for (let placeX = 0; placeX < boardSize; placeX++) {
		for (let placeY = 0; placeY < boardSize; placeY++) {
			if (grid[placeY][placeX].stone) continue;
			for (let n = 1; n <= boardSize; n++) {
				for (let sx = 0; sx < boardSize - n; sx++) {
					for (let sy = 0; sy < boardSize - n; sy++) {
						if (
							winner(
								applyMove(
									grid,
									{ spinSize: n, spinX: sx, spinY: sy, placeX, placeY },
									"Black",
									"",
								),
							) === "Black"
						)
							sols++;
					}
				}
			}
		}
	}
	return sols;
}

export function generatePuzzle(): Grid {
	let grid = randomWin();
	let sx: number;
	let sy: number;
	let n: number;
	let px: number;
	let py: number;
	do {
		n = (Math.random() * 4 + 2) | 0;
		sx = (Math.random() * (boardSize - n + 1)) | 0;
		sy = (Math.random() * (boardSize - n + 1)) | 0;
	} while (winner(unspin(grid, sx, sy, n)) === "Black");
	grid = unspin(grid, sx, sy, n);
	const iota = [...Array(boardSize).keys()];
	const locations = iota.flatMap((y) =>
		iota.map((x) => [x, y]).filter(([x, y]) => grid[y][x].stone),
	);
	[px, py] = locations[(Math.random() * locations.length) | 0];
	grid[py][px].stone = undefined;
	const ss = solutions(grid);
	let failures = 0;
	for (let i = 0; i < 20; i++) {
		const rx = (Math.random() * boardSize) | 0;
		const ry = (Math.random() * boardSize) | 0;
		if (rx === px && ry === py) continue;
		if (grid[ry][rx].stone) continue;
		grid[ry][rx].stone = {
			color: Math.random() > 0.5 ? "Black" : "White",
			label: "/",
			rotation: 0,
		};
		if (solutions(grid) > ss) {
			grid[ry][rx].stone = undefined;
			failures++;
			if (failures === 200) return grid;
		}
	}
	return grid;
}
