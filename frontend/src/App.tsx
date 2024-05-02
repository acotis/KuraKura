// import { useState } from "react";
import "./App.css";
import { Board } from "./Board";

function App() {
  return (
    <>
      <h1>kurakura!</h1>
      <Board
        grid={[
          [undefined, undefined, undefined, undefined, undefined, undefined],
          [undefined, undefined, undefined, undefined, undefined, undefined],
          [undefined, undefined, "white", undefined, undefined, undefined],
          [undefined, undefined, undefined, "black", undefined, undefined],
          [undefined, undefined, undefined, undefined, undefined, undefined],
          [undefined, undefined, undefined, undefined, undefined, undefined],
        ]}
        active="white"
        onMove={(move) => {
          console.log(move);
        }}
      />
    </>
  );
}

export default App;
