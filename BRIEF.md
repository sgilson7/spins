# Brief

Stage 1. The person's own words come first; the rest can be written by the agent from them and read by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

From `BRIEF.md` as the person wrote it:

> Description of the tool/game here:
>
> I am trying to recreate the minigame "Wheels" in the video game "Sea of Stars" so if there is any information that you can gather on that game to help in the following task, please use it, however, please default and/or overwrite any of those specifications with the following specifications I've provided below.
>
>
> This is a table-top game that you might play in a tavern with a random opponent to pass the time and have some fun called "Spins." Spins is a turn based game where 2 players sit across from each other and each control 2 "Legends" of a certain "proficiency" - a Legend of a certain proficiency can either be in the Red position on the board (called the Red Legend from here on) and or in the Blue position on the board (called the Blue Legend from here on). The board is split in half so that each player controls a half, and one player will be controlled by the human playing this game (called "User" from here on) and another by a computer. Each player has 10 points of health and use their Legends to deal damage to their opponent, but each player can generate Barricade segments that stack vertically each for 1 unit of height, blocking 1 point of damage to themselves for each Barricade segment, and subsequently destroying 1 segment of the Barricade for each point of damage if the Barricade is hit. The height of the Barricade dynamically changes as Barricade segments are added or destroyed, with a max possible height of 5 Barricade segments. The Legends have an energy bar split into a number of segments - when the bar filled with generated segments of their respective red and blue energy types, the Legend performs a set action based on their proficiency. To generate energy or Barricade segments, on each player's half of the board they have a set of 5 consecutive individually spinning wheels, like a slot machine, in the center that have panels of symbols encircling each of the wheels so that the wheels display 1 panel of symbol(s) face up at a time which will be considered the current, active symbol(s). Each player's Red and Blue Legends reside on the left and right of their set of wheels, respectively. The symbol(s) displayed on each panel of the slot-machine-like wheels are of 1 of 3 types, (1) 1 or 2 red energy symbols, (2) 1 or 2 blue energy symbols, or (3) 1 or 2 Barricade symbols. Each player's actions resolve on the same turn at the end of the turn, and each turn consists of each player getting 3 separate spins of the set of wheels. The Red or Blue Legend can perform their action if they have a full energy bar of energy segments of their respective color at the end of the turn. 1 energy segment of a certain color is generated if 3 energy symbols of that color are the active symbols on the player that controls them's set of wheels at the end of the turn, with another segment being generated for each additional number of active symbols displayed. This is termed "3oak+" and an example would be that after a player has taken their 3 spins for the turn, and their wheels display 3 currently active red symbols they generate 1 red energy segment for their Red Legend - if their wheels displayed 4 active red symbols they would generate 2 red energy segments, 5 active red symbols would generate 3 red energy segments, and so on. The same "3oak+" scheme applies to any active Barricade symbols on the wheels. If a Legend performs their action, their energy bar is reset to being empty with 0 energy segments, but the number of energy segments persist between turns. On each separate spin of the set of wheels, each player can toggle whether they would like a particular wheel to be locked so that it is not spun on the next of the 3 spins in the turn and the currently active symbols on that wheel remain active - if a wheel is locked on one of those 3 spins it can be unlocked on a later spin if chosen so by the player. Each Legend's a energy bar will maintain the  Each player's sets of wheels are partially covered so that they are only visible to the player on the respective side of the board, and therefore cannot be seen by the player sitting across from them, while the turn is ongoing and the players are spinning their wheels. The game will generally be displayed from the User's perspective, so that they see their set of wheels at all times. At the end of the turn, both player's wheels are fully visible to both players and any Legends on the board take their actions, if they can based on the results of spinning the wheels. When Legends attack, they will generally first be blocked by any number Barricade segments before they can do damage to the opponent's health - although there are exceptions based on the specific actions of each proficiency. For example, when the Knight proficiency (further explained below) performs its action, it attacks once - if the opponent has at least 1 Barricade segment, the entire attack will be taken by the Barricade segment(s) and the opponent will take no damage, but if the opponent has 0 Barricade segments the Knight's attack will deal full damage to the opponent's health.
>
>
> Template for proficiency action:
>
> -(Proficiency name): (number of energy segments needed to take their action)
> ---(what this proficency's action does)
>
> The following list describes each proficiency following the above template:
>
> -Knight: 3
> ---Attacks. Does 3 points of damage to the opponent's health or 3 points of damage to the opponent's Barricade. Will always hit the opponent's barricade first if they have any Barricade segments.
>
> -Wizard: 5
> ---Attacks 2 separate times. Each attack does 2 points of damage to the opponent's health or 2 points of damage to the opponent's Barricade. The first attack will hit the opponent's Barricade if they have any Barricade segments, and the second attack flies above any Barricade segments so that it always hits the opponent's health regardless of how many Barricade segments the opponent has.
>
> -Bowman: 4
> ---Attacks. Does 3 points of damage to the opponent's health or 1 point of damage to the opponent's Barricade. Flies at a height of 3 units so that if the opponent's Barricade is at 2 segments or less, it hits the opponent's health anyways.
>
>
> What does fun mean in this game:
>
>
> Inspirations:
> The Wheels minigame featured in Sea of Stars (game)
>
> Useful links:
>
> >

The session request that started this build (2026-10-24), word for word:

> get yourself familiarized with the builder repository and clone it from https://github.com/sgilson7/builder-setting into this repo then remove the .git. You will be using builder setting to help constrain and direct your actions as you help act as as a video game  creator and co-designer / developer, with the goal of deploying it to a github pages so its freely browser accessible, with an emphasis on optimization for running in the browser.  Please also see https://sgilson7.github.io/projects/ for examples of other browser based tools / games built in builder like the PDF redactor, youtube to mp3, long time coming, extremely lifelike fishing  etc. Also see ptonkle.github.io for games built using builder by my good friend. First thing is I want you to do is interpret builder, and write an analysis of how you will use it to help you develop this game, and write into the first tab of the design journal, the design journal process is described in builder; when you write the design journal, use the local ape-race directories design journal as a representative of a good journaling process. you may access the other builder made games within the documents directory on this host machine.
>
> The game I want is specified in brief.md. I Want special care placed on making the gameplay as similar to sea of stars wheels as possible as a baseline to build from, and use the brief for the rest. start by creating a tab in the design log on how you will use builder to make this game. then make the 2nd tab your intrepretation of the brief, then the third is a synthesis of the brief into one coherent game with mockups, flow diagrams, menus, gameplay mockups, meta game progression mockups, design decisions for the designer to make. then, the 4th tabe should be a 2d version of the game built out in an mvp in low-baller style,  then the 5th tab is a 3d version of the game in protection of the spheres / chumworld style, the 6th is 2d in a pixellated style, 7th is comparison of mockup vs 2d vs 3d vs 2d pixel. make all 3 versions of the game accessible on the website, whre you have explicit permission to make deploy sgilson7.github.io. do not spin up any subagents, i want to minimize token use if possible. you have permission to create a public github repo and publig github pages deployment. now please begin, dont stop until the mockups (tabs 1-3) are ready for my review, then completing the mvp means finishing all 7 tabs and posting all 3 versions, which shoyld have byte identical engines, but just differnt views

## The tool/game
What is your high level vision for the game: 

*Drafted by the agent from the description, for you to read (journal tab 2, SP-Q questions):* Spins is a two-player tavern table game against a computer opponent, played in the browser. Each player spins five wheels up to three times a turn, locking wheels between spins, and the symbols showing at the end of the turn fill two Legends' energy bars and build a Barricade. Full Legends act at the end of the turn, both sides at once, until one player's health reaches 0. The rules follow Sea of Stars' Wheels where the description is silent, and the description wherever it speaks. One rules engine, three views: a flat 2D view in Lowballer's style, a 3D view in the style of Protection of the Spheres and CHUMWORLD, and a 2D pixel view.

Target Audience: 

*Drafted:* anyone visiting sgilson7.github.io who wants a short game of chance and choice; a match lasts about five minutes.

What devices do you want to play this on:

*Drafted:* desktop browsers (Chromium, Firefox, WebKit) with mouse or keyboard; phones and tablets by touch, in landscape and portrait.

## Done means
Conditions that define when the first minimum viable product is completed and ready for evaluation:

*Drafted by the agent, for you to read:*

- The MVP is one match of Spins against a computer opponent, playable start to finish in each of the three views: 2D (Lowballer style), 3D (Protection of the Spheres / CHUMWORLD style) and 2D pixel.
- All three views load the same WebAssembly engine file; its bytes are identical because it is one file, and a test checks that each view's page names it.
- Every rule in the description holds and has a test worked out by hand: 10 health; Barricade at most 5; 5 wheels, 3 spins a turn, locks toggled between spins; 3oak+ for red, blue and Barricade counted by symbols; Knight 3, Wizard 5, Bowman 4 with the described attacks; both players resolve at the end of the turn; the opponent's wheels hidden until then.
- Journal tabs 1 to 7 are written, and the game is live at a public GitHub Pages address with a link from sgilson7.github.io.

Define targets for the final product:
Define low-targets (features you'd be embarrased not to have)

*Drafted:* the rules above with no known rules bug; readable symbols on a phone; locks that are obvious; a turn-end reveal you can follow; a computer opponent that locks wheels sensibly; win and loss screens and a rematch.

Define medium targets  (intent of what you are making)

*Drafted:* Sea of Stars' feel: the XP tiles and Bronze, Silver and Gold evolution, a pick of two Legends before the match, sound for spins, locks and hits, and a tavern of opponents of rising skill.

high targets (polish that means the game is ready to show others)

*Drafted:* animated attacks per proficiency (the Knight's swing into the Barricade, the Wizard's second bolt over it, the Bowman's arrow at height 3), a meta-game of tavern champions and better wheels, a tutorial match.

sretch goals (not sure are possible at the moment  but would be cool)
- 

*Drafted:* more proficiencies (Sea of Stars' Engineer, Priest, Assassin); two people on one device; online play (outside the default build, R7).

## Decisions that stay with the person
Define decisions that the coding agent may NOT make without first checking with the human co-designer.

- 

*Drafted:* the rules wherever the description and Sea of Stars disagree, or neither speaks (journal tab 3, SP-Q); the names of opponents, taverns and anything else a player reads that the agent generated; whether the game keeps progress between visits (D17); any third-party asset or sound; which view is the default on the site.

## Accessibility
How do assets carry information in ways besides colour (shape, pattern, brightness, text): 

*Drafted:* red energy is a square, blue energy is a diamond (Sea of Stars' shapes), Barricade is a hammer; a double symbol shows two of the shape. Locked wheels show a padlock and a shifted position, not only a tint. Health, energy and Barricade show numbers beside their bars.

Input devices tool/game must work with:

*Drafted:* mouse, keyboard (1 to 5 lock wheels, Space spins, Enter ends the turn), touch.

## Style

A description of the look of the tool / game:

A tavern-themed game set in the middle ages. UI should be old middle age style parchment, scrolls, wood, and other simple materials. Board and pieces should be fully made of carved wood of various styles to make pieces visually distinct.

Inspirations for the style:

*From the session request:* view 1, Lowballer's flat look (thick ink outlines, offset shadows, warm flat colour); view 2, the 3D look of Protection of the Spheres and CHUMWORLD (rounded low-poly figures, soft light, ink outlines); view 3, a 2D pixel look. All three keep the tavern, parchment and carved wood.

## Words
Specific terms that must appear in the game:

*From the description:* Spins, Legend, proficiency, Red Legend, Blue Legend, Barricade, Barricade segment, energy bar, energy segment, wheel, panel, active symbol, spin, lock, 3oak+, health, turn, Knight, Wizard, Bowman. They go into `GLOSSARY.md` before any string uses them.

## Autonomy

One paragraph: what the agent may do alone, what it must ask about, and what it may never do. State whether the plan is approved in advance and whether the agent may publish.

The model is pre-approved to ask the user to get github access, and when it does, it creates a public github repo for version control and a public github pages deployment to a website.

*The grant from the session request, word for word (2026-10-24):* "make all 3 versions of the game accessible on the website, whre you have explicit permission to make deploy sgilson7.github.io. do not spin up any subagents, i want to minimize token use if possible. you have permission to create a public github repo and publig github pages deployment." The agent stops for review once journal tabs 1 to 3 are ready ("dont stop until the mockups (tabs 1-3) are ready for my review"); after that the plan counts as approved and the agent builds the MVP, tabs 4 to 7, without stopping. It may create the public repository `sgilson7/spins`, publish it to GitHub Pages, and add the game to sgilson7.github.io. It uses no subagents. It may not loosen a Builder requirement or a boundary test, and may not add a server, an account, analytics or an off-origin request.

## Journal

Design journal will follow the ape-race style, producing mockups, style guides, flow diagrams, maps, animations, comparisons against inspirations, using the text above in this brief.

Location: `default` (`../spins-journal/`), seven tabs in the order the session request gives.

## Allowed core dependencies

`serde`, `serde_json` (reference default). Add others only here, with the reason.
