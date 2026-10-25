"""Spins' browser interactions, run by check.py in every engine.

check.py has loaded the chooser (index.html) and let it idle. From there
this opens each of the three views and plays one whole match in each, the
way a person would: seat two Legends, sit with an opponent, spin, lock,
spin, end turns until someone's health reaches none. `?fast` sets the
views' waits to none; nothing else changes.

What is checked, worked out from the brief rather than read from the game:
- every view loads the same engine file (SP-1);
- the opponent's wheels are covered while you spin, and show at the end of
  the match (SP-4, B11);
- a match ends with one side at no health (B3), and the result scroll says
  win, loss or draw;
- locking a wheel keeps its face through the next spin (B10).
"""

import json
import pathlib

STRINGS = json.loads((pathlib.Path(__file__).resolve().parents[2] / "data" / "strings.json").read_text())
VIEWS = ["2d", "pixel", "3d"]
BUSY = "body[data-busy='false']"


def click(page, sel):
    page.click(sel)
    page.wait_for_selector(BUSY, state="attached", timeout=20000)


def play(page, check, view, base, engine):
    page.goto(f"{base}/{view}/index.html?fast")
    page.wait_for_selector("body[data-ready='true']", state="attached", timeout=20000)
    check(page.get_attribute("body", "data-screen") == "title", f"{view}: the title screen is shown")
    check(STRINGS["title"][""] in page.inner_text("#menu"), f"{view}: the sign reads the game's name")
    if view == "3d" and page.get_attribute("body", "data-gl") == "none":
        check(page.inner_text("#notice") == STRINGS["page"]["nogl"], f"3d ({engine}): no WebGL 2, and the page says so")
        return
    click(page, "button[data-action='screen.show'][data-value='legends']")
    check(page.locator("section.kind-card").count() == 3, f"{view}: three proficiencies to choose from")
    click(page, "button[data-action='screen.show'][data-value='opponents']")
    click(page, "button[data-action='opponent.pick'][data-value='tobble']")
    check(page.get_attribute("body", "data-screen") == "match", f"{view}: the match starts")
    click(page, "button[data-action='match.spin']")
    if view != "3d":
        check(page.locator(".half.them .wheels.covered").count() == 1, f"{view}: the opponent's wheels are covered")
        before = page.get_attribute(".half.you .strip", "data-face")
    click(page, "button[data-action='match.lock'][data-value='0']")
    click(page, "button[data-action='match.spin']")
    if view != "3d":
        after = page.get_attribute(".half.you .strip", "data-face")
        check(before is not None and before == after, f"{view}: a locked wheel keeps its face through a spin")
    turns = 0
    while page.get_attribute("body", "data-screen") == "match" and turns < 80:
        if page.locator("button[data-action='match.end']:not([disabled])").count():
            click(page, "button[data-action='match.end']")
        else:
            click(page, "button[data-action='match.spin']")
        turns += 1
    check(page.get_attribute("body", "data-screen") == "over", f"{view}: the match ends ({turns} actions)")
    heading = page.inner_text("#over h2")
    check(heading in (STRINGS["over"]["win"], STRINGS["over"]["loss"], STRINGS["over"]["draw"]), f"{view}: the result reads {heading!r}")
    if view != "3d":
        check(page.locator(".half.them .wheels.covered").count() == 0, f"{view}: the opponent's wheels show at the end")
        healths = page.eval_on_selector_all(".plaque-in .health .num", "ns => ns.map(n => n.textContent)")
        check("0" in healths, f"{view}: one side's health is at none: {healths}")
    click(page, "button[data-action='over.rematch']")
    check(page.get_attribute("body", "data-screen") == "match", f"{view}: a rematch starts")


def run(page, check, engine):
    base = page.url.rsplit("/", 1)[0]
    wasm = set()
    page.on("request", lambda r: wasm.add(r.url.split("/")[-1]) if ".wasm" in r.url else None)
    check(page.locator(".choices a").count() == 3, "the chooser links the three views")
    check(page.inner_text("#title") == STRINGS["title"][""], "the chooser shows the name")
    for view in VIEWS:
        play(page, check, view, base, engine)
    check(len(wasm) == 1, f"every view loaded the same engine file: {sorted(wasm)}")
