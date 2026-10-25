// The chooser at /spins/: three links to the three views of one game. Its
// words come from data/strings.json like every other page's.
const words = await fetch("data/strings.json").then((r) => r.json());
document.getElementById("title").textContent = words.title[""];
document.getElementById("tagline").textContent = words.title.tagline;
document.getElementById("pick").textContent = words.page.pick;
document.getElementById("same").textContent = words.page.same;
document.querySelectorAll("[data-word]").forEach((n) => {
  n.textContent = words.page[n.dataset.word];
});
document.body.dataset.ready = "true";
