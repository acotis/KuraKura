import { createContext, ReactNode } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";

export type KuraRequest = "MakeUser";
export type KuraResponse = "Hello";

function parseResponse(json: unknown): KuraResponse | undefined {
  if (json === "Hello") {
    return "Hello";
  } else {
    console.warn("Unexpected KuraResponse:", json);
    return undefined;
  }
}

interface WebSocketContextType {
  send: (message: KuraRequest) => void;
  last: KuraResponse | undefined;
  readyState: ReadyState;
}

export const WebSocketContext = createContext<WebSocketContextType | null>(null);

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
