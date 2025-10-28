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
	const ctx = useContext(WebSocketContext);
	if (!ctx) throw new Error("Need websocket context");
	const { send, readyState } = ctx;

	const [started, setStarted] = useState(false);
	const [messages, setMessages] = useState<KuraResponse[]>([]);
	const [blackPlayerName, setBlackPlayerName] = useState<string | undefined>(undefined);
	const [whitePlayerName, setWhitePlayerName] = useState<string | undefined>(undefined);
	const [roomJoinId, setRoomJoinId] = useState<string | undefined>(undefined);
	const [isHost, setIsHost] = useState(room === "create");

	useEffect(() => {
		if (readyState === WebSocket.OPEN && !started) {
			if (room === "create") {
				send({
					CreateRoom: {
						name: playerName,
						parameters: {
							host_plays_black: true,
							grid_size: 6,
							win_length: 4,
						},
					},
				});
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

			// Handle room state updates
			if ("Ok" in last) {
				const ok = last.Ok;
				if (typeof ok === "object" && ok !== null) {
					if ("RoomCreated" in ok || "RoomJoined" in ok) {
						const data = "RoomCreated" in ok ? ok.RoomCreated : ok.RoomJoined;
						const { player_names, room_id } = data.room_state;
						const { host_plays_black } = data.room_state.game_state;

						// Store the room ID for the invite link (used as join ID)
						setRoomJoinId(room_id);

						// First player is host
						if (player_names.length >= 1) {
							if (host_plays_black) {
								setBlackPlayerName(player_names[0]);
							} else {
								setWhitePlayerName(player_names[0]);
							}
						}
						// Second player is guest
						if (player_names.length >= 2) {
							if (host_plays_black) {
								setWhitePlayerName(player_names[1]);
							} else {
								setBlackPlayerName(player_names[1]);
							}
						}
					} else if ("TurnAccepted" in ok) {
						// Sync the board state from the server's authoritative state
						const { room_state } = ok.TurnAccepted;
						const { board, turn } = room_state.game_state;

						// Convert server board format to our Grid format
						const newGrid: Grid = board.map(row =>
							row.map(cell => ({
								stone: cell.stone
									? {
											color: cell.stone.color,
											label: "", // Server doesn't send labels, we'll need to derive from turn number
											rotation: 0,
									  }
									: undefined,
							}))
						);

						setGrid(newGrid);
						setMoveNumber(turn);
						// Determine whose turn it is based on turn number and host_plays_black
						const { host_plays_black } = room_state.game_state;
						const isBlackTurn = turn % 2 === 1;
						setActive(isBlackTurn ? "Black" : "White");
					}
				}
			}
		}
	}, [last]);

	const [grid, setGrid] = useState<Grid>(
		new Array(boardSize).fill(undefined).map(() =>
			new Array(boardSize).fill(undefined).map(() => ({
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
			black={blackPlayerName ? { name: blackPlayerName } : undefined}
			white={whitePlayerName ? { name: whitePlayerName } : undefined}
			roomJoinId={roomJoinId}
			isHost={isHost}
		/>
	);
}
