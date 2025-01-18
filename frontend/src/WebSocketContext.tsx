import { createContext, type ReactNode } from "react";
import useWebSocket, { type ReadyState } from "react-use-websocket";
import type { Color } from "./types";

type UserId = string;
type RoomId = string;
type Unit = [];

export interface TurnDetails {
	player: Color;
	play_row: number;
	play_col: number;
	spin_ul_row: number;
	spin_ul_col: number;
	spin_size: number;
	spin_dir: "CW" | "CCW";
}

export type KuraRequest =
	| { CreateRoom: { name: UserId } }
	| { JoinRoom: { name: UserId; room: RoomId } }
	| { TakeTurn: { turn: TurnDetails } };

export type KuraResponse = { Ok: KuraOk } | { Err: KuraErr };

export type KuraOk =
	| { RoomCreated: { id: RoomId } }
	| { JoinedAsPlayer: Unit }
	| { JoinedAsSpectator: Unit }
	| { TurnAccepted: Unit };

export type KuraErr =
	| "AccountNotFound"
	| "AlreadyLoggedIn"
	| "AlreadyInARoom"
	| "NotInARoom"
	| "NotLoggedIn"
	| "RoomNotFound"
	| "AccountAlreadyHasRoom" // Todo: add a paramater giving the room ID?
	| "RoomAlreadyHasGuest" // (probably don't add such a parameter here for the player ID) (definitely not, that would reveal someone else's API key)
	| "NameTooLong"
	| "AccountDoesntHaveRoom"
	| "RoomDoesntHaveGuest"
	| "AccountPlayedWrongColor"
	| { InvalidTurn: unknown }
	| "NotImplemented"
	| "InvalidJson";

function parseResponse(json: unknown): KuraResponse | undefined {
	if (!json) return undefined;

	if (json && typeof json === "object") {
		if ("Ok" in json && typeof json.Ok === "object" && json.Ok) {
			return json as KuraResponse;
		}
		if ("Err" in json && typeof json.Err === "object" && json.Err) {
			console.warn("Parsed Err:", json.Err);
			return json as KuraResponse;
		}
	}
	console.warn("Unexpected KuraResponse:", json);
	return undefined;
}

export interface WebSocketContextType {
	send: (message: KuraRequest) => void;
	last: KuraResponse | undefined;
	readyState: ReadyState;
}

export const WebSocketContext = createContext<WebSocketContextType | null>(
	null,
);

// Create a provider component
export const WebSocketProvider = ({ children }: { children: ReactNode }) => {
	const socketUrl = "ws://localhost:3000";

	const { sendJsonMessage, lastJsonMessage, readyState } =
		useWebSocket(socketUrl);

	console.log({ lastJsonMessage, readyState });

	const value = {
		send: (msg: KuraRequest) => {
			console.debug("Sending", msg);
			sendJsonMessage(msg);
			sendJsonMessage("DebugLog");
		},
		last: parseResponse(lastJsonMessage),
		readyState,
	};

	return (
		<WebSocketContext.Provider value={value}>
			{children}
		</WebSocketContext.Provider>
	);
};
