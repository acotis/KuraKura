import { createContext, ReactNode } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";

type UserId = string;
type RoomId = string;
type Unit = Record<string, never>;

export interface TurnDetails {
  play_row: number;
  play_col: number;
  spin_ul_row: number;
  spin_ul_col: number;
  spin_size: number;
  spin_dir: "CW" | "CCW";
}

export type KuraRequest =
  | { CreateUser: Unit }
  | { SetName: { auth: UserId; name: string } }
  | { CreateRoom: { auth: UserId } }
  | { JoinRoom: { auth: UserId; room: RoomId } }
  | { TakeTurn: { auth: UserId; details: TurnDetails } };

export type KuraResponse =
  | { UserCreated: { id: UserId } }
  | { NameSet: Unit }
  | { RoomCreated: { id: RoomId } }
  | { RoomJoined: Unit }
  | { TurnTaken: Unit };

function parseResponse(json: unknown): KuraResponse | undefined {
  if (json && typeof json === "object") {
    if (
      "UserCreated" in json &&
      json.UserCreated &&
      typeof json.UserCreated === "object" &&
      "id" in json.UserCreated &&
      typeof json.UserCreated.id === "string"
    ) {
      return { UserCreated: { id: json.UserCreated.id } };
    }
  } else {
    console.warn("Unexpected KuraResponse:", json);
    // But, ah, what the heck:
    return json as KuraResponse;
  }
}

interface WebSocketContextType {
  send: (message: KuraRequest) => void;
  last: KuraResponse | undefined;
  readyState: ReadyState;
}

export const WebSocketContext = createContext<WebSocketContextType | null>(
  null
);

// Create a provider component
export const WebSocketProvider = ({ children }: { children: ReactNode }) => {
  const socketUrl = "ws://localhost:3000";

  const { sendJsonMessage, lastJsonMessage, readyState } =
    useWebSocket(socketUrl);

  const value = {
    send: (msg: KuraRequest) => sendJsonMessage(msg),
    last: parseResponse(lastJsonMessage),
    readyState,
  };

  return (
    <WebSocketContext.Provider value={value}>
      {children}
    </WebSocketContext.Provider>
  );
};
