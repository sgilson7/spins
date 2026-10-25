#!/usr/bin/env python3
"""The pixel view's sprites, drawn as text grids, written into
web/pixel/index.html as SVG symbols between the sprites markers.

Each letter is a palette role; the page's stylesheet colours the role
(.f-red, .f-seat …), so the sprites hold no colour. Runs of one letter in a
row become one rectangle. Run it after editing a grid:

    python3 tools/pixel/sprites.py
"""
import pathlib
import re

ROLE = {"K": "ink", "R": "red", "r": "red-hi", "B": "blue", "b": "blue-hi", "G": "gilt", "W": "wood",
        "D": "wood-dk", "N": "walnut", "O": "oak", "H": "birch", "T": "steel", "P": "skin", "S": "seat"}

SPRITES = {
    "sym-red": """
.KKKKKKKK.
KRRRRRRRRK
KRrrRRRRRK
KRrRRRRRRK
KRRRRRRRRK
KRRRRRRRRK
KRRRRRRRRK
KRRRRRRRRK
KRRRRRRRRK
.KKKKKKKK.""",
    "sym-blue": """
.....K.....
....KBK....
...KbBBK...
..KbBBBBK..
.KBBBBBBBK.
KBBBBBBBBBK
.KBBBBBBBK.
..KBBBBBK..
...KBBBK...
....KBK....
.....K.....""",
    "sym-barricade": """
.KKKKKKKKK.
KWWWWWWWWWK
KWWWWWWWWWK
KDDDDDDDDDK
.KKKKDKKKK.
....KDK....
....KDK....
....KDK....
....KDK....
....KDK....
....KKK....""",
    "sym-lock": """
...KKKK...
..K....K..
..K....K..
.KKKKKKKK.
KGGGGGGGGK
KGGGKKGGGK
KGGGKKGGGK
KGGGGGGGGK
KGGGGGGGGK
.KKKKKKKK.""",
    "fig-knight": """
.....KKKKKK.....
....KTTTTTTK....
...KTTTTTTTTK...
...KTTKTTKTTK...
...KTTTKKTTTK...
...KPPPPPPPPK...
...KPKPPPPKPK...
...KPPPPPPPPK...
....KKPPPPKK....
..KKSSSSSSSSKK..
.KOOKSSSSSSKTK..
.KOOKSSSSSSKTK..
.KOOKSSSSSSKTK..
.KOOKSSSSSSKTK..
..KKKSSSSSSKTK..
....KSSSSSSKKK..
...KSSSSSSSSK...
.KKKKKKKKKKKKKK.
KDDDDDDDDDDDDDDK
.KKKKKKKKKKKKKK.""",
    "fig-wizard": """
.........K......
........KSK.....
.......KSSK.....
......KSSSSK....
.....KSSSSSSK...
....KSSSSSSSSK..
..KKHHHHHHHHHHKK
...KPPPPPPPPK.GG
...KPKPPPPKPK.GG
...KPHHHHHHPK.D.
....KHHHHHHK..D.
..KKSKHHHHKSKKD.
.KSSSSKHHKSSSKD.
.KSSSSSSSSSSSKD.
.KSSSSSSSSSSSKD.
..KSSSSSSSSSSKD.
..KSSSSSSSSSSKD.
.KKKKKKKKKKKKKK.
KDDDDDDDDDDDDDDK
.KKKKKKKKKKKKKK.""",
    "fig-bowman": """
....KKKKKKK.....
...KNNNNNNNK....
..KNNNNNNNNNK...
..KNKPPPPPKNK.K.
..KNPKPPPKPNK..N
..KNPPPPPPPNK..N
...KNPPPPPNK...N
....KKKKKKK....N
...KSSSSSSSK...N
..KSSSSSSSSSK..N
.KNKSSSSSSSK..N.
.KNKSSSSSSSK..N.
.KNKSSSSSSSK..N.
..KKSSSSSSSK.N..
...KSSSSSSSK.N..
...KSSSSSSSKN...
..KSSSSSSSSSK...
.KKKKKKKKKKKKKK.
KDDDDDDDDDDDDDDK
.KKKKKKKKKKKKKK.""",
    "art-champion": """
K....KK....K
KG..KGGK..GK
KGGKGGGGKGGK
KGGGGGGGGGGK
KGGGGGGGGGGK
KKKKKKKKKKKK""",
}
# the menu cards show the same figures
for p in ("knight", "wizard", "bowman"):
    SPRITES[f"art-{p}"] = SPRITES[f"fig-{p}"]


def symbol(name, grid):
    rows = grid.strip("\n").split("\n")
    w = max(len(r) for r in rows)
    assert all(len(r) == w for r in rows), f"{name}: rows differ in width"
    rects = []
    for y, row in enumerate(rows):
        x = 0
        while x < w:
            c = row[x]
            if c == ".":
                x += 1
                continue
            end = x
            while end < w and row[end] == c:
                end += 1
            rects.append(f'<rect x="{x}" y="{y}" width="{end - x}" height="1" class="f-{ROLE[c]}"/>')
            x = end
    return f'<symbol id="{name}" viewBox="0 0 {w} {len(rows)}" shape-rendering="crispEdges">{"".join(rects)}</symbol>'


def main():
    page = pathlib.Path(__file__).resolve().parents[2] / "web" / "pixel" / "index.html"
    text = page.read_text()
    block = "\n".join(symbol(n, g) for n, g in SPRITES.items())
    new, n = re.subn(r"(<!-- sprites:start -->\n).*?(<!-- sprites:end -->)",
                     lambda m: m.group(1) + block + "\n" + m.group(2), text, flags=re.S)
    assert n == 1, "the sprites markers are missing from web/pixel/index.html"
    page.write_text(new)
    print(f"{len(SPRITES)} sprites written to {page}")


if __name__ == "__main__":
    main()
