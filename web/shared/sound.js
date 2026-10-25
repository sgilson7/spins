// Short synthesised sounds (T5, from SP-Q19): each is an oscillator and an
// envelope whose numbers come from data/sounds.json. Sound starts with the
// first click, as browsers require, and the reader can turn it off.
let ctx = null;
let on = true;

export function makeSound(cfg) {
  const voice = (name) => cfg.voices[name];
  function play(name) {
    const v = voice(name);
    if (!on || !v) return;
    if (!ctx) ctx = new AudioContext();
    const t = ctx.currentTime;
    for (let i = 0; i < (v.repeat || 1); i += 1) {
      const at = t + (i * (v.gap_ms || 0)) / cfg.ms_per_second;
      const end = at + v.ms / cfg.ms_per_second;
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = v.type;
      osc.frequency.setValueAtTime(v.from, at);
      osc.frequency.exponentialRampToValueAtTime(v.to, end);
      gain.gain.setValueAtTime(v.gain, at);
      gain.gain.exponentialRampToValueAtTime(cfg.silence, end);
      osc.connect(gain).connect(ctx.destination);
      osc.start(at);
      osc.stop(end);
    }
  }
  // a sound that fails must never stop the game
  const safe = (name) => {
    try {
      play(name);
    } catch {
      on = false;
    }
  };
  return {
    event(kind) {
      safe(cfg.events[kind]);
    },
    ui(kind) {
      safe(cfg.ui[kind]);
    },
    toggle() {
      on = !on;
      return on;
    },
    get on() {
      return on;
    },
  };
}
