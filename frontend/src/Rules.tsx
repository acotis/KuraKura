import Board from "./Board";
import { applyMove } from "./logic";
import { boardLinesFor, type Grid } from "./types";

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
    })),
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
    "",
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
    "",
);

export default function Rules() {
    return <div className="fcc gap-4 text-center">
        <h2 className="text-2xl font-bold">Rules</h2>
        <p>
            Kurakura is a two-player game. It's{" "}
            <a href="https://en.wikipedia.org/wiki/Gomoku">five-in-a-row</a> with a
            twist:
        </p>
        <div className="grid md:grid-cols-2 gap-4">
            <div className="fcc">
                <div className="relative">
                    <Board
                        grid={example2}
                        tileSize={40}
                        active={undefined}
                        moveNumber={0}
                        onMove={() => { }}
                    />
                    <div className="outline-dashed outline-white absolute left-[48px] w-[40px] top-[88px] h-[40px] rounded-full shadow-xl" />
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
                        onMove={() => { }}
                    />
                    <div className="outline-dashed outline-white absolute left-[88px] w-[120px] top-[128px] h-[120px] shadow-xl" />
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

}