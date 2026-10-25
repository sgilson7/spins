// Plays the end of a turn. The core returns the turn's events with the
// numbers after each one; this file copies each event's numbers onto the
// table being shown, hands the event to the view, and waits the event's
// time from data/view.json. It works nothing out: every value it writes was
// in an event.
import { clone } from "./core.js?v=feec326eef01";

const wait = (ms) => new Promise((done) => setTimeout(done, ms));

function copy(t, e) {
  const side = t.sides[e.side];
  const legend = side ? side.legends[e.slot] : null;
  switch (e.kind) {
    case "reveal":
      t.sides.forEach((s, i) => {
        s.wheels = e.wheels[i];
        s.covered = false;
      });
      break;
    case "barricade":
      side.barricade = e.value;
      break;
    case "energy":
      legend.energy = e.value;
      break;
    case "xp":
      legend.xp = e.value;
      break;
    case "evolve":
      legend.level = e.level;
      legend.level_name = e.label;
      break;
    case "act":
      legend.energy = e.energy;
      break;
    case "hit":
      t.sides[e.foe].barricade = e.barricade;
      t.sides[e.foe].health = e.health;
      break;
    case "burst":
      t.sides[e.foe].health = e.health;
      break;
    default:
      break;
  }
}

export function makePlayer({ draw, event, view, fast }) {
  let shown = null;
  let skipping = false;
  const scale = (ms) => (fast || skipping ? view.fast_scale * ms : ms);

  return {
    get shown() {
      return shown;
    },
    skip() {
      skipping = true;
    },
    // Shows a new table; with events, plays them first. Resolves when done.
    async show(table, events, spun) {
      if (!table) {
        shown = null;
        return;
      }
      if (!events || !events.length) {
        shown = clone(table);
        draw(shown, { spun, playing: false });
        if (spun) await wait(scale(view.spin_ms));
        return;
      }
      skipping = false;
      // your wheels land on their last faces before anything is counted
      const t = clone(shown || table);
      t.sides[0].wheels = table.sides[0].wheels.map((w, i) => ({
        ...w,
        locked: t.sides[0].wheels[i] ? t.sides[0].wheels[i].locked : false,
      }));
      draw(t, { spun, playing: true });
      if (spun) await wait(scale(view.spin_ms + view.stagger_ms * t.sides[0].wheels.length));
      for (const e of events) {
        copy(t, e);
        const ms = scale(view.event_ms[e.kind] || view.event_ms.other);
        event(e, t, ms);
        draw(t, { spun: false, playing: true });
        await wait(ms);
      }
      await wait(scale(view.settle_ms));
      shown = clone(table);
      draw(shown, { spun: false, playing: false });
    },
  };
}
