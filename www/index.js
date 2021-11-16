import { Evosim } from "wasm-evosim3";

const evosim = Evosim.new();

evosim.start();

const renderLoop = () => {
  evosim.render();

  requestAnimationFrame(renderLoop);
};
renderLoop();