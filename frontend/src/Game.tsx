// import { useState } from "react";
import { useContext, useEffect, useState } from "react";
import { type Color, type Grid, boardSize } from "./types";
import { applyMove } from "./logic";
import {
	type KuraResponse,
	type TurnDetails,
	WebSocketContext,
} from "./WebSocketContext";
import GameView from "./GameView";

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
			})),
		),
	);

	const [active, setActive] = useState<Color>("Black");
	const [moveNumber, setMoveNumber] = useState(1);

	if (readyState === WebSocket.CONNECTING) {
		return "Connecting...";
	}

	if (readyState === WebSocket.CLOSED || readyState === WebSocket.CLOSING) {
		return "Connection lost.";
	}

	return (
		<GameView
			messages={messages}
			grid={grid}
			active={active}
			moveNumber={moveNumber}
			onMove={(move) => {
				setGrid(applyMove(grid, move, active, moveNumber.toString()));
				setActive(active === "Black" ? "White" : "Black");
				setMoveNumber(moveNumber + 1);
				const turnDetails: TurnDetails = {
					player: active,
					play_row: move.placeY,
					play_col: move.placeX,
					spin_ul_row: move.spinY,
					spin_ul_col: move.spinX,
					spin_size: move.spinSize,
					spin_dir: "CW",
				};
				send({ TakeTurn: { turn: turnDetails } });
			}}
			black={{ name: "Rain" }}
			white={{ name: "Fire" }}
		/>
	);
}
