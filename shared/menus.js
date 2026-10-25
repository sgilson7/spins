// Draws the core's panels (title, rules, Legends, tavern, match over) as
// page elements, without knowing what any panel means. Each view's stylesheet
// gives the kinds (sign, card, notice, scroll, bar) their look, and each
// view's page defines the pictures a panel's `art` names (#art-knight …).
import { make } from "./core.js?v=feec326eef01";

const SVG = "http://www.w3.org/2000/svg";

export function art(name, className) {
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("class", className);
  svg.setAttribute("aria-hidden", "true");
  const use = document.createElementNS(SVG, "use");
  use.setAttribute("href", `#${name}`);
  svg.append(use);
  return svg;
}

export function button(a, onAction) {
  const b = make("button", a.style ? `act ${a.style}` : "act", a.label);
  b.disabled = !a.enabled;
  b.dataset.action = a.name;
  b.dataset.value = a.value;
  b.addEventListener("click", () => onAction(a.name, a.value));
  return b;
}

// A heading or label that is a glossary term shows the definition on hover.
export function hint(node, hints) {
  const h = hints.find((x) => x.term.toLowerCase() === node.textContent.toLowerCase());
  if (h) {
    node.dataset.term = h.term;
    node.title = h.definition;
  }
  return node;
}

export function drawPanels(box, panels, hints, onAction) {
  box.replaceChildren(
    ...panels.map((p) => {
      const s = make("section", `panel kind-${p.kind}`);
      s.id = p.id;
      if (p.mark) s.dataset.mark = p.mark;
      if (p.art) s.append(art(`art-${p.art}`, "art"));
      if (p.heading) s.append(hint(make("h2", "", p.heading), hints));
      if (p.text) s.append(hint(make("p", "lead", p.text), hints));
      if (p.lines) {
        const ul = make("ul", "lines");
        p.lines.forEach((line) => ul.append(make("li", "", line)));
        s.append(ul);
      }
      if (p.actions) {
        const row = make("div", "actions");
        p.actions.forEach((a) => row.append(button(a, onAction)));
        s.append(row);
      }
      return s;
    }),
  );
}
