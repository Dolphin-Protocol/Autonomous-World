import init, {
  set_wasm,
  get_player_speed,
  set_player_speed,
} from "./dist/autonomous-game.js";

// state
// wasm setup
async function impl_run() {
  let wbg = await init();
  miniquad_add_plugin({
    register_plugin: (a) => (a.wbg = wbg),
    on_init: () => {
      set_wasm(wasm_exports);
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
  console.log(get_player_speed());
};

window.setPlayerSpeed = function (speed) {
  set_player_speed(speed);
};
