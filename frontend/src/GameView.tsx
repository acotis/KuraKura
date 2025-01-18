import Board from "./Board";
import BoardStone from "./BoardStone";
import type { Color, Grid, Move } from "./types";
import type { KuraResponse } from "./WebSocketContext";

export interface Player {
	name: string;
}

export interface GameViewProps {
	messages: KuraResponse[];
	grid: Grid;
	active: Color;
	moveNumber: number;
	onMove: (move: Move) => void;
	black: Player | undefined;
	white: Player | undefined;
}

function PlayerCard({
	color,
	name,
	active,
}: { color: Color; name: string | undefined; active: boolean }) {
	return (
		<div className="">
			<div className="flex flex-row gap-2 text-lg">
				<div className="relative w-8 h-8">
					<BoardStone stone={{ color, label: "", rotation: 0 }} />
				</div>

				{name ? (
					<span className={active ? "font-bold" : ""}>{name}</span>
				) : (
					"Waiting for player..."
				)}
			</div>
		</div>
	);
}

export default function GameView({
	messages,
	grid,
	active,
	moveNumber,
	onMove,
	black,
	white,
}: GameViewProps) {
	return (
		<div className="flex flex-col border p-4 gap-2">
			<div className="flex flex-row gap-2">
				<Board
					grid={grid}
					tileSize={40}
					active={active}
					moveNumber={moveNumber}
					onMove={onMove}
				/>
				<div className="flex flex-col gap-2">
					<PlayerCard
						color="black"
						name={black?.name}
						active={active === "black"}
					/>
					<PlayerCard
						color="white"
						name={white?.name}
						active={active === "white"}
					/>
				</div>
			</div>
			<div className="text-xs">
				{messages.map((m, i) => (
					// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
					<pre key={i}>{JSON.stringify(m)}</pre>
				))}
			</div>
		</div>
	);
}
