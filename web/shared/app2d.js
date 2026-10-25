// The two 2D views (Lowballer and pixel) run this same file. They differ in
// their page's pictures and their stylesheet, never in what they draw.
import { boot, call, el } from "./core.js";
import { drawPanels } from "./menus.js";
import { makeBoard } from "./board.js";
import { makePlayer } from "./play.js";
import { makeSound } from "./sound.js";
import { frame } from "./chrome.js";

const { game, words, view, sounds, fast } = await boot();
const sound = makeSound(sounds);
let ui = JSON.parse(game.ui()).ok;
let busy = false;

const board = makeBoard(el("table"), view, ui.hints, act);
const player = makePlayer({
  view,
  fast,
  draw: (t, o) => board.draw(t, o),
  event: (e) => {
    board.log(e.text, e.kind);
    board.flash(e);
    sound.event(e.kind);
  },
});

function cue(name) {
  if (name === "match.spin") sound.ui("spin");
  else if (name === "match.lock") sound.ui("lock");
  else sound.ui("button");
}

async function act(name, value) {
  if (busy) return;
  cue(name);
  const next = call(game, name, value);
  if (next.error) {
    el("notice").textContent = next.error;
    return;
  }
  busy = true;
  document.body.dataset.busy = "true";
  await show(next, name === "match.spin");
  busy = false;
  document.body.dataset.busy = "false";
}

async function show(next, spun) {
  ui = next;
  el("notice").textContent = ui.notice || "";
  const table = el("table");
  table.hidden = !ui.table;
  if (!ui.table) board.reset();
  el("menu").hidden = true;
  document.body.dataset.screen = ui.table && ui.screen === "over" ? "playing" : ui.screen;
  document.body.dataset.playing = ui.events ? "true" : "false";
  await player.show(ui.table, ui.events, spun);
  document.body.dataset.playing = "false";
  document.body.dataset.screen = ui.screen;
  document.body.dataset.turn = ui.table ? `${ui.table.turn}` : "";
  drawPanels(el("menu"), ui.panels, ui.hints, act);
  el("menu").hidden = !ui.panels.length;
}

frame({ words, view, sound, player, act, current: () => ui.table });
await show(ui, false);
document.body.dataset.ready = "true";
