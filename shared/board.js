// The table as page elements, for the two 2D views. It draws the table the
// core described: health, Barricade, the Legends' bars and XP, your wheels
// and their locks, the opponent's cover. A wheel is a strip of its panels
// in a window; the core's face index says which panel sits in the middle,
// and CSS slides the strip there. Each view's page defines the pictures
// (#sym-red, #fig-knight …) and its stylesheet the look.
import { make } from "./core.js?v=feec326eef01";
import { art, button, hint } from "./menus.js?v=feec326eef01";

function panelNode(p) {
  const n = make("div", `face sym-${p.sym}`);
  n.dataset.count = p.count;
  if (p.gilded) n.dataset.gilded = "true";
  for (let i = 0; i < p.count; i += 1) n.append(art(`sym-${p.sym}`, "sym"));
  return n;
}

function bar(cls, filled, total) {
  const n = make("div", cls);
  for (let i = 0; i < total; i += 1) n.append(make("span", i < filled ? "cell on" : "cell"));
  return n;
}

export function makeBoard(root, view, hints, onAction) {
  let parts = null;

  function legendNode(l, words) {
    const n = make("div", `legend seat-${l.seat} prof-${l.prof} level-${l.level}`);
    n.append(art(`fig-${l.prof}`, "figure"));
    const label = make("div", "label");
    label.append(hint(make("strong", "", l.name), hints), make("span", "level", l.level_name));
    n.append(label);
    const energy = bar("energy", l.energy, l.cost);
    energy.setAttribute("aria-label", `${words.energy} ${l.energy}/${l.cost}`);
    energy.append(make("b", "num", `${l.energy}/${l.cost}`));
    n.append(energy);
    const xp = bar("xp", l.xp, l.xp_max);
    xp.setAttribute("aria-label", `${words.xp} ${l.xp}/${l.xp_max}`);
    n.append(xp);
    return n;
  }

  function plaque(s, words) {
    const n = make("div", "plaque-in");
    n.append(make("strong", "name", s.name));
    const h = bar("health", s.health, s.health_max);
    h.setAttribute("aria-label", `${words.health} ${s.health}/${s.health_max}`);
    h.append(make("b", "num", `${s.health}`));
    n.append(h);
    return n;
  }

  function barricade(s, words) {
    const n = make("div", "planks");
    n.setAttribute("aria-label", `${words.barricade} ${s.barricade}/${s.barricade_max}`);
    for (let i = 0; i < s.barricade_max; i += 1) n.append(make("span", i < s.barricade ? "plank on" : "plank"));
    n.append(hint(make("b", "num", `${words.barricade} ${s.barricade}`), hints));
    return n;
  }

  // Puts a strip at a panel; `turn` slides it there the long way round.
  function place(strip, face, panels, turn) {
    strip.classList.add("still");
    strip.style.setProperty("--pos", face);
    if (!turn) return;
    void strip.offsetWidth;
    strip.classList.remove("still");
    strip.style.setProperty("--pos", face + panels * view.spin_copy);
  }

  function skeleton() {
    const half = (cls) => {
      const h = make("section", cls);
      const p = { half: h, plaque: make("div", "plaque"), legends: [make("div", "seat"), make("div", "seat")] };
      p.wheels = make("div", "wheels");
      p.barricade = make("div", "barricade");
      const row = make("div", "row");
      row.append(p.legends[0], p.wheels, p.legends[1]);
      h.append(p.plaque, row, p.barricade);
      p.cache = { covered: null, wheels: [] };
      return p;
    };
    const sides = [half("half you"), half("half them")];
    const controls = make("div", "controls");
    const log = make("ol", "log");
    root.replaceChildren(sides[1].half, make("div", "seam"), sides[0].half, controls, log);
    return { sides, controls, log };
  }

  function drawWheels(side, p, s, words, spun, locks) {
    const c = p.cache;
    if (c.covered !== s.covered || c.wheels.length !== s.wheels.length) {
      c.covered = s.covered;
      p.wheels.classList.toggle("covered", s.covered);
      if (s.covered) {
        const cover = make("div", "cover");
        cover.append(make("strong", "", words.covered));
        c.pips = make("div", "pips");
        cover.append(c.pips);
        p.wheels.replaceChildren(cover);
        c.wheels = [];
      } else {
        c.wheels = s.wheels.map((w, i) => {
          const node = make("div", "wheel");
          node.style.setProperty("--w", i);
          node.style.setProperty("--panels", w.panels.length);
          const win = make("div", "window");
          const strip = make("div", "strip");
          for (let k = 0; k < view.strip_copies; k += 1) w.panels.forEach((x) => strip.append(panelNode(x)));
          win.append(strip);
          const slot = make("div", "lock-slot");
          node.append(win, slot);
          return { node, strip, slot, locked: false, fresh: true };
        });
        p.wheels.replaceChildren(...c.wheels.map((x) => x.node));
      }
    }
    if (s.covered) {
      c.pips.replaceChildren(bar("spins", s.spins_used, s.spins_max));
      return;
    }
    s.wheels.forEach((w, i) => {
      const x = c.wheels[i];
      const turn = (spun && side === 0 && !x.locked) || (x.fresh && side === 1);
      x.fresh = false;
      x.locked = w.locked;
      x.strip.dataset.face = w.face;
      x.node.classList.toggle("locked", w.locked);
      x.slot.replaceChildren();
      if (locks) {
        const lb = button(locks[i], onAction);
        lb.setAttribute("aria-pressed", w.locked ? "true" : "false");
        lb.setAttribute("aria-label", locks[i].label);
        lb.dataset.key = view.keys.lock[i];
        lb.prepend(art("sym-lock", "lock-icon"));
        x.slot.append(lb);
      }
      if (turn || x.face !== w.face) {
        x.face = w.face;
        requestAnimationFrame(() => place(x.strip, w.face, w.panels.length, turn));
      }
    });
  }

  function controls(box, t, playing) {
    const counts = make("div", "counts");
    t.counts.forEach((k) => {
      const n = make("span", k.segments ? `count sym-${k.sym} pays` : `count sym-${k.sym}`);
      n.append(art(`sym-${k.sym}`, "sym"), make("span", "", k.text));
      counts.append(n);
    });
    const row = make("div", "actions");
    t.actions.forEach((a) => {
      const b = button(a, onAction);
      if (playing && a.name !== "match.leave") b.disabled = true;
      row.append(b);
    });
    box.replaceChildren(counts, make("p", "prompt", t.prompt), row);
  }

  return {
    reset() {
      parts = null;
    },
    draw(t, { spun, playing }) {
      if (!parts || !root.contains(parts.log)) parts = skeleton();
      const words = t.words;
      parts.log.setAttribute("aria-label", words.log);
      t.sides.forEach((s, side) => {
        const p = parts.sides[side];
        p.plaque.replaceChildren(plaque(s, words));
        p.legends.forEach((box, slot) => box.replaceChildren(legendNode(s.legends[slot], words)));
        p.barricade.replaceChildren(barricade(s, words));
        drawWheels(side, p, s, words, spun, side === 0 && !playing ? t.locks : null);
      });
      controls(parts.controls, t, playing);
      root.dataset.playing = playing ? "true" : "false";
    },
    log(text, kind) {
      if (!parts || !text) return;
      parts.log.prepend(make("li", `ev-${kind}`, text));
      while (parts.log.children.length > view.log_lines) parts.log.lastChild.remove();
    },
    flash(e) {
      root.dataset.event = e.kind;
      root.dataset.eventSide = e.foe !== undefined ? e.foe : e.side;
    },
  };
}
