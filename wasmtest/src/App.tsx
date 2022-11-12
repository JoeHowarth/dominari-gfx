import React, { useEffect, useState } from "react";
import "./App.css";
import * as DominariGfx from "dominari-gfx";

function App() {
  return (
    <div className="App">
      {/* {wasm ? wasm.init() : null} */}
      <header className="App-header">
        <canvas id="game" className="game" width={1080} height={720} />
        <button onClick={() => DominariGfx.move_up()}>Up</button>
      </header>
    </div>
  );
}

export default App;
