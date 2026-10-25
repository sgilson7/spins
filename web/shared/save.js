// The one file that names browser storage (T3, from SP-Q17; BUILDER-SPEC.md
// records the change to D17). It keeps one item, the text the core writes:
// wins and losses at the tavern and whether oak wheels were won. The page
// never reads inside it; the core does.
const KEY = "spins.save";

export function readSave() {
  try {
    return localStorage.getItem(KEY) || "";
  } catch {
    return "";
  }
}

export function writeSave(text) {
  try {
    if (text) localStorage.setItem(KEY, text);
    else localStorage.removeItem(KEY);
  } catch {
    // private windows refuse storage; the game plays on without a record
  }
}
