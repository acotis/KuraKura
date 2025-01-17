import { useContext } from "react";
import { WebSocketContext } from "./WebSocketContext";
import { useDarkMode } from "usehooks-ts";
import Rules from "./Rules";

export default function Home() {
	const ctx = useContext(WebSocketContext);
	if (!ctx) throw new Error("Need websocket context");
	const { isDarkMode, toggle } = useDarkMode();

	return (
		<main
			className="flex flex-col items-center gap-12 p-8"
			data-theme={isDarkMode ? "night" : "bumblebee"}
		>
			<h1 className="font-bold text-5xl flex items-center tracking-tight">
				{/* biome-ignore lint/a11y/noSvgWithoutTitle: <explanation> */}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					strokeWidth={2}
					stroke="currentColor"
					className="inline-block mr-2 w-10 h-10"
				>
					<path
						strokeLinecap="round"
						strokeLinejoin="round"
						d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
					/>
				</svg>
				<span className="text-secondary">kurakura</span>
			</h1>

			<button
				type="button"
				className="btn btn-ghost absolute left-4 top-4"
				onClick={() => toggle()}
			>
				Theme
			</button>

			<div className="fc gap-4">
				<label className="input input-bordered flex items-center gap-2">
					{/* biome-ignore lint/a11y/noSvgWithoutTitle: <explanation> */}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						strokeWidth={1.5}
						stroke="currentColor"
						className="w-6 h-6"
					>
						<path
							strokeLinecap="round"
							strokeLinejoin="round"
							d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
						/>
					</svg>
					<input type="text" className="grow" placeholder="Name" />
				</label>
				<button type="button" onClick={() => {}} className="btn btn-primary">
					Start a game
				</button>
			</div>
			<Rules />
		</main>
	);
}
