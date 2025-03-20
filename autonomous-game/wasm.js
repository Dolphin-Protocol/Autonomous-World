import init, { set_wasm, GameState } from "./dist/autonomous-game.js";

// state
let gameState = null;
async function initGameState() {
  try {
    gameState = new GameState();
    // Make the function globally available
    window.checkPlayerSpeed = function () {
      if (gameState) {
        console.log(gameState.speed);
      } else {
        console.log("GameState not initialized yet");
      }
    };
  } catch (error) {
    console.error("Failed to initialize GameState:", error);
  }
}

// wasm setup
async function impl_run() {
  let wbg = await init();
  miniquad_add_plugin({
    register_plugin: (a) => (a.wbg = wbg),
    on_init: () => {
      set_wasm(wasm_exports);
      initGameState();
    },
    version: "0.0.1",
    name: "wbg",
  });
  load("./dist/autonomous-game_bg.wasm");
}

// export globally
window.run = function () {
  document.getElementById("run-container").remove();
  document.getElementById("glcanvas").removeAttribute("hidden");
  document.getElementById("glcanvas").focus();
  impl_run();
};

window.checkPlayerSpeed = function () {
  console.log(gameState.speed);
};

window.setPlayerSpeed = async function (speed) {
  gameState.set_player_speed(speed);
};
