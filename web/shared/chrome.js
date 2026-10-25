// The page's own frame, the same in every view: the view links, the sound
// switch, the skip button and the keys. Its words come from data/strings.json.
import { el } from "./core.js";

export function frame({ words, view, sound, player, act, current }) {
  document.querySelectorAll("[data-word]").forEach((n) => {
    n.textContent = words.page[n.dataset.word];
  });
  document.documentElement.style.setProperty("--spin-ms", `${view.spin_ms}ms`);
  document.documentElement.style.setProperty("--stagger-ms", `${view.stagger_ms}ms`);
  const toggle = el("sound");
  toggle.addEventListener("click", () => {
    toggle.textContent = sound.toggle() ? words.page.sound_on : words.page.sound_off;
    toggle.setAttribute("aria-pressed", sound.on ? "true" : "false");
  });
  toggle.textContent = words.page.sound_on;
  el("skip").addEventListener("click", () => player.skip());
  addEventListener("keydown", (e) => {
    if (document.body.dataset.screen !== "match" || e.target.closest("input, textarea")) return;
    const i = view.keys.lock.indexOf(e.key);
    const t = current();
    if (!t) return;
    if (i >= 0 && t.locks[i]) act(t.locks[i].name, t.locks[i].value);
    else if (e.key === view.keys.spin) act("match.spin", "");
    else if (e.key === view.keys.end) act("match.end", "");
    else return;
    e.preventDefault();
  });
}
