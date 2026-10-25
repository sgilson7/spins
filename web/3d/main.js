// The 3D view. The same engine as the 2D views (SP-1); the table is drawn
// by WebGL 2 from instances the Rust scene builds out of the core's table,
// and a page-element layer on top carries the numbers, the lock buttons and
// the controls. When nothing moves, the frame loop stops.
import { boot, call, el, make } from "../shared/core.js";
import { drawPanels, button, art, hint } from "../shared/menus.js";
import { makePlayer } from "../shared/play.js";
import { makeSound } from "../shared/sound.js";
import { frame } from "../shared/chrome.js";
import { makeRenderer } from "./gl.js";

const { game, words, view, sounds, fast, numbers } = await boot();
const N = JSON.parse(numbers());
const [vertex, fragment] = await Promise.all([
  fetch("../data/shaders/scene.vert").then((r) => r.text()),
  fetch("../data/shaders/scene.frag").then((r) => r.text()),
]);
const sound = makeSound(sounds);
const canvas = el("view");
const renderer = makeRenderer(canvas, N, { vertex, fragment }, (id) => game.mesh(id));
let ui = JSON.parse(game.ui()).ok;
let busy = false;
let running = false;
let table = null;

function loop(now) {
  game.render(now);
  renderer.setUniforms(game.uniforms(), game.clear_color());
  renderer.frame(game.mats(), game.draws(), game.values());
  if (game.busy(now)) requestAnimationFrame(loop);
  else running = false;
}

function kick() {
  if (!renderer || running) return;
  running = true;
  requestAnimationFrame(loop);
}

function resize() {
  if (!renderer) return;
  const [w, h] = game.resize(canvas.clientWidth, canvas.clientHeight, window.devicePixelRatio || 1);
  renderer.resize(w, h);
  placeLocks();
  kick();
}

function placeLocks() {
  const a = game.anchors();
  let k = 0;
  el("locks").querySelectorAll("button").forEach((b) => {
    b.style.left = `${a[k]}%`;
    k += 1;
    b.style.top = `${a[k]}%`;
    k += 1;
  });
}

function bar(cls, filled, total) {
  const n = make("span", cls);
  for (let i = 0; i < total; i += 1) n.append(make("i", i < filled ? "on" : ""));
  return n;
}

function hud(box, s, w) {
  const top = make("div", "who");
  top.append(make("strong", "", s.name));
  const h = bar("health", s.health, s.health_max);
  h.setAttribute("aria-label", `${w.health} ${s.health}/${s.health_max}`);
  top.append(h, make("b", "", `${s.health}`));
  const wall = hint(make("span", "wall", `${w.barricade} ${s.barricade}`), ui.hints);
  top.append(wall);
  const legends = make("div", "legends");
  s.legends.forEach((l) => {
    const n = make("div", `leg seat-${l.seat}`);
    n.append(art(`art-${l.prof}`, "face"));
    const t = make("div", "txt");
    t.append(hint(make("strong", "", l.name), ui.hints), make("span", "level", l.level_name));
    const e = bar("energy", l.energy, l.cost);
    e.setAttribute("aria-label", `${w.energy} ${l.energy}/${l.cost}`);
    t.append(e, bar("xp", l.xp, l.xp_max));
    n.append(t);
    legends.append(n);
  });
  box.replaceChildren(top, legends);
}

function draw(t, { spun, playing }) {
  table = t;
  const err = game.set_table(JSON.stringify(t), spun, performance.now(), view.spin_ms);
  if (err) el("notice").textContent = err;
  hud(el("hud-you"), t.sides[0], t.words);
  hud(el("hud-them"), t.sides[1], t.words);
  const locks = el("locks");
  locks.replaceChildren(
    ...(playing || !t.sides[0].wheels.length
      ? []
      : t.locks.map((a, i) => {
          const b = button(a, act);
          b.setAttribute("aria-pressed", t.sides[0].wheels[i].locked ? "true" : "false");
          // the core's label stays as the name; the button shows the key that also locks it
          b.setAttribute("aria-label", a.label);
          b.title = a.label;
          b.textContent = view.keys.lock[i];
          b.prepend(art("sym-lock", "lock-icon"));
          return b;
        })),
  );
  placeLocks();
  const c = el("controls");
  const counts = make("div", "counts");
  t.counts.forEach((k) => {
    const n = make("span", k.segments ? "count pays" : "count");
    n.append(art(`sym-${k.sym}`, "sym"), make("span", "", k.text));
    counts.append(n);
  });
  const row = make("div", "actions");
  t.actions.forEach((a) => {
    const b = button(a, act);
    if (playing && a.name !== "match.leave") b.disabled = true;
    row.append(b);
  });
  c.replaceChildren(counts, make("p", "prompt", t.prompt), row);
  kick();
}

const player = makePlayer({
  view,
  fast,
  draw,
  event: (e, t, ms) => {
    game.event(JSON.stringify(e), performance.now(), ms);
    const log = el("log");
    if (e.text) log.prepend(make("li", `ev-${e.kind}`, e.text));
    while (log.children.length > view.log_lines) log.lastChild.remove();
    sound.event(e.kind);
    kick();
  },
});

async function act(name, value) {
  if (busy) return;
  if (name === "match.spin") sound.ui("spin");
  else if (name === "match.lock") sound.ui("lock");
  else sound.ui("button");
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
  const fresh = !ui.table && next.table;
  ui = next;
  el("notice").textContent = ui.notice || (renderer ? "" : words.page.nogl);
  el("play").hidden = !ui.table || !renderer;
  if (fresh) {
    el("log").replaceChildren();
    resize();
  }
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

frame({ words, view, sound, player, act, current: () => table });
new ResizeObserver(resize).observe(canvas);
await show(ui, false);
document.body.dataset.gl = renderer ? "on" : "none";
document.body.dataset.ready = "true";
