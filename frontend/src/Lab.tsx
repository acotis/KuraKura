import { useDarkMode } from "usehooks-ts";
import Game from "./Game";
import Board from "./Board";
import { type Grid, Move } from "./types";
import { useEffect, useState } from "react";
import { generatePuzzle } from "./puzzle";

export default function Lab() {
	const { isDarkMode, toggle } = useDarkMode();
	const [puzzleGrid, setPuzzleGrid] = useState<Grid>([[]]);
	useEffect(() => {
		setPuzzleGrid(generatePuzzle());
	}, []);

	return (
		<main
			className="flex flex-col items-center gap-4 p-8"
			data-theme={isDarkMode ? "night" : "bumblebee"}
		>
			<h1 className="text-3xl italic">~kurakura lab~</h1>
			<button type="button" className="btn btn-ghost" onClick={() => toggle()}>
				Theme
			</button>

			<hr className="border w-full" />
			{/*
			<Board
				tileSize={40}
				grid={puzzleGrid}
				whoseTurn={"Black"}
				active={"Black"}
				moveNumber={0}
				onMove={() => {}}
			/> */}

			<Game playerName="Laqme" room="create" />
		</main>
	);
}
