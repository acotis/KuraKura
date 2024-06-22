import { useContext, useState } from "react";
import Board from "./Board";
import { applyMove } from "./logic";
import { Grid, boardLinesFor } from "./types";
import { WebSocketContext, useWebSocketContext } from "./WebSocketContext";

const grid = `......
7.....
.....6
.8.45.
..23..
..1...`.split("\n");

const example1: Grid = grid.map((row, y) =>
  [...row].map((c, x) => ({
    stone:
      c === "."
        ? undefined
        : { color: +c % 2 ? "black" : "white", rotation: 0, label: "" },
    lines: boardLinesFor(x, y, 6),
  }))
);

const example2 = applyMove(
  example1,
  {
    placeX: 1,
    placeY: 2,
    spinX: 2,
    spinY: 3,
    spinSize: 1,
  },
  "black",
  ""
);

const example3 = applyMove(
  example2,
  {
    placeX: 1,
    placeY: 2,
    spinX: 2,
    spinY: 3,
    spinSize: 3,
  },
  "black",
  ""
);

export default function Home() {
  const { send } = useContext(WebSocketContext)!;

  return (
    <main className="flex flex-col items-center gap-12 p-8">
      <h1 className="font-bold text-5xl flex items-center tracking-tight">
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

      <div className="fc gap-4">
        <label className="input input-bordered flex items-center gap-2">
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
        <button
          onClick={() => {
            send("MakeUser");
          }}
          className="btn btn-primary"
        >
          Start a game
        </button>
      </div>
      <div className="fcc gap-4 text-center">
        <h2 className="text-2xl font-bold">Rules</h2>
        <p>
          Kurakura is a two-player game. It's{" "}
          <a href="https://en.wikipedia.org/wiki/Gomoku">Gomoku</a> with a
          twist.
        </p>
        <div className="grid md:grid-cols-2 gap-4">
          <div className="fcc">
            <div className="relative">
              <Board
                grid={example2}
                tileSize={40}
                active={undefined}
                moveNumber={0}
                onMove={() => {}}
              />
              <div className="outline-dashed outline-white absolute left-[48px] w-[40px] top-[88px] h-[40px] rounded-full shadow-xl"></div>
            </div>
            <p className="my-4">
              On your turn, <strong>place</strong> a stone…
            </p>
          </div>
          <div className="fcc">
            <div className="relative">
              <Board
                grid={example3}
                tileSize={40}
                active={undefined}
                moveNumber={0}
                onMove={() => {}}
              />
              <div className="outline-dashed outline-white absolute left-[88px] w-[120px] top-[128px] h-[120px] shadow-xl"></div>
            </div>
            <p className="my-4">
              then <strong>spin</strong> a square region.
            </p>
          </div>
        </div>
        <p>
          Line up five stones horizontally, vertically, or diagonally to win!
        </p>
      </div>
    </main>
  );
}
