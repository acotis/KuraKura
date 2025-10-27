import { useDarkMode } from "usehooks-ts";
import Game from "./Game";

export default function Lab() {
	const { isDarkMode, toggle } = useDarkMode();

	// Check for ?join=<gameId> parameter
	const params = new URLSearchParams(window.location.search);
	const joinId = params.get("join");

	const room = joinId ? { joinId } : "create";

	return (
		<main
			className="flex flex-col items-center gap-4 p-8"
			data-theme={isDarkMode ? "night" : "bumblebee"}
		>
			<h1 className="text-3xl italic">~kurakura lab~</h1>
			<button type="button" className="btn btn-ghost" onClick={() => toggle()}>
				Theme
			</button>

			<Game playerName="Laqme" room={room} />
		</main>
	);
}
