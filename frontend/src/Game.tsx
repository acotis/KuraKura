// import { useState } from "react";
import { useCallback, useContext, useEffect, useState } from "react";
import Board from "./Board";
import { type Color, type Grid, boardLinesFor, boardSize } from "./types";
import { applyMove } from "./logic";
import { KuraResponse, WebSocketContext } from "./WebSocketContext";

interface GameProps {
	playerName: string;
	room: { joinId: string } | "create";
}

export default function Game({ playerName, room }: GameProps) {
	// const params = new URLSearchParams(location.search);

	const ctx = useContext(WebSocketContext);
	if (!ctx) throw new Error("Need websocket context");
	const { send, readyState } = ctx;

	const [started, setStarted] = useState(false);
	const [messages, setMessages] = useState<KuraResponse[]>([]);

	useEffect(() => {
		if (readyState === WebSocket.OPEN && !started) {
			if (room === "create") {
				send({ CreateRoom: { name: playerName } });
			} else {
				send({ JoinRoom: { name: playerName, room: room.joinId } });
			}
			setStarted(true);
		}
	}, [playerName, room, started, send, readyState]);

	const { last } = ctx;
	useEffect(() => {
		if (last) {
			setMessages((m) => [...m, last]);
		}
	}, [last]);

	const [grid, setGrid] = useState<Grid>(
		new Array(boardSize).fill(undefined).map((_, y) =>
			new Array(boardSize).fill(undefined).map((_, x) => ({
				stone: undefined,
				lines: boardLinesFor(x, y, boardSize),
			})),
		),
	);
	const [active, setActive] = useState<Color>("black");
	const [moveNumber, setMoveNumber] = useState(1);

	if (readyState !== WebSocket.OPEN) {
		return "Connecting...";
	}

	return (
		<div>
			{messages.map((m, i) => (
				// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
				<pre key={i}>{JSON.stringify(m)}</pre>
			))}
			<Board
				grid={grid}
				tileSize={40}
				active={active}
				moveNumber={moveNumber}
				onMove={(move) => {
					setGrid(applyMove(grid, move, active, moveNumber.toString()));
					setActive(active === "black" ? "white" : "black");
					setMoveNumber(moveNumber + 1);
				}}
			/>
		</div>
	);
}
