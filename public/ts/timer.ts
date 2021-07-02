import init, {
  timer,
} from "../pkg/wasm_timer.js";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

// function sleep(ms: number): Promise<void> {
  // return new Promise<void>(resolve => setTimeout(resolve, ms))
// }

// (globalThis as any).sleep = sleep;

async function run() {
  const wasm = await fetch("public/pkg/wasm_timer_bg.wasm");
  await init(wasm);
  timer(10);
}

run();

