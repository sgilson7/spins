// Starts the one engine every view shares (SP-1): the same module, from the
// same address, for 2D, 3D and pixel. Everything the page shows comes back
// from `act` as the core's description of the screen.
import init, { Game, numbers } from "../pkg/tool_shim.js?v=feec326eef01";
import { readSave, writeSave } from "./save.js?v=feec326eef01";

export const el = (id) => document.getElementById(id);

export function make(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

export const clone = (v) => JSON.parse(JSON.stringify(v));

export async function boot() {
  await init();
  const [strings, glossary, view, sounds] = await Promise.all([
    fetch("../data/strings.json?v=feec326eef01").then((r) => r.text()),
    fetch("../data/glossary.md?v=feec326eef01").then((r) => r.text()),
    fetch("../data/view.json?v=feec326eef01").then((r) => r.json()),
    fetch("../data/sounds.json?v=feec326eef01").then((r) => r.json()),
  ]);
  const seed = new Uint32Array(1);
  crypto.getRandomValues(seed);
  const game = new Game(strings, glossary, readSave(), seed[0]);
  const fast = new URLSearchParams(location.search).has(view.fast_flag);
  return { game, words: JSON.parse(strings), view, sounds, fast, numbers };
}

// Sends one action; stores the save text when the core hands one over.
export function call(game, name, value) {
  const r = JSON.parse(game.act(name, value));
  if (r.error) return { error: r.error };
  if (r.ok.save !== undefined) writeSave(r.ok.save);
  return r.ok;
}
