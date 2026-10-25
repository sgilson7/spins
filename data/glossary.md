# Glossary

The source of truth for what each word in this tool means. Strings, tests, the plan and the README use a term in the sense given here. The page shows a definition when a reader hovers the term.

Write each entry as a `### Term` heading, then one or two plain sentences, then an optional `Not:` line listing words that must not be used for it. A number a rule decides is a `{placeholder}` the core fills, so the glossary cannot disagree with the rules. Keep entries in alphabetical order. `crates/core/tests/copy.rs` checks the format, the placeholders and the `Not:` words; `TONE.md` governs the prose.

### 3oak+

Three or more symbols of one kind on the active panels at the end of a turn. Three make one segment, and each symbol after the third makes one more. A double panel counts as two symbols.

Not: combo

### Barricade

Segments stacked in front of a player's health, at most {barricade.max} high. Each segment blocks one point of damage and is destroyed by it.

Not: wall, bulwark, shield

### Blue Legend

The Legend in the Blue position, on the right of its player's wheels. Blue energy symbols, drawn as diamonds, fill its energy bar.

### Bowman

A proficiency whose energy bar holds {bowman.cost} segments. Its arrow flies at height {bowman.height}: it hits health for {bowman.damage} over a lower Barricade, and takes {bowman.chip} segment from a Barricade at that height or more.

Not: archer

### Energy bar

A Legend's bar of energy segments. When it is full at the end of a turn, the Legend acts and the bar empties.

Not: rod, meter

### Energy segment

One part of an energy bar, made by {oak}oak+ of the Legend's colour. Segments stay between turns.

Not: charge

### Evolve

A Legend that reaches {xp.max} XP evolves, from Bronze to Silver to Gold. Each level adds one to every damage number of that Legend.

Not: level up

### Gilded panel

A red or blue panel with a gilt ring. Each one showing at the end of a turn gives its Legend one XP, with no {oak}oak+ needed.

Not: XP tile

### Grade

The wood a set of wheels is carved from. Oak wheels carry more doubles than pine ones.

### Health

What a player loses when an attack gets through. Each player starts with {health}; reaching none loses the match.

Not: HP, crown, life

### Knight

A proficiency whose energy bar holds {knight.cost} segments. It attacks once for {knight.damage}, into the Barricade if there is any, otherwise into health.

Not: warrior

### Legend

A carved figure seated beside a player's wheels. Each player has two, a Red Legend and a Blue Legend, of two proficiencies.

Not: hero, figurine, unit

### Lock

A wheel that is locked keeps its active panel through the next spin. Locks change between the spins of a turn.

Not: hold

### Panel

One of the {panels} faces carved around a wheel. A panel shows one or two symbols of one kind: red energy, blue energy or Barricade.

Not: tile, slot

### Proficiency

What a Legend does when its energy bar fills: Knight, Wizard or Bowman.

Not: class, role

### Red Legend

The Legend in the Red position, on the left of its player's wheels. Red energy symbols, drawn as squares, fill its energy bar.

### Spin

Turning every unlocked wheel. Each player has {spins} spins a turn, and the first turns all {wheels} wheels.

Not: roll, pull

### Turn

Both players take their spins, then both sides' Barricades rise, energy fills and full Legends act together.

Not: round

### Wheel

One of the {wheels} carved wheels in front of each player. The panel facing up is its active panel.

Not: reel

### Wizard

A proficiency whose energy bar holds {wizard.cost} segments. It attacks twice for {wizard.damage}: the first into any Barricade, the second over it into health.

Not: mage

### XP

What a Legend gains from gilded panels and, {xp.action} at a time, from acting. A Gold Legend that fills its XP again does {burst} damage straight to health.

Not: experience
