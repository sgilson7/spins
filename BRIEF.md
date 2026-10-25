# Brief

Stage 1. The person's own words come first; the rest can be written by the agent from them and read by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

Description of the tool/game here:

I am trying to recreate the minigame "Wheels" in the video game "Sea of Stars" so if there is any information that you can gather on that game to help in the following task, please use it, however, please default and/or overwrite any of those specifications with the following specifications I've provided below.


This is a table-top game that you might play in a tavern with a random opponent to pass the time and have some fun called "Spins." Spins is a turn based game where 2 players sit across from each other and each control 2 "Legends" of a certain "proficiency" - a Legend of a certain proficiency can either be in the Red position on the board (called the Red Legend from here on) and or in the Blue position on the board (called the Blue Legend from here on). The board is split in half so that each player controls a half, and one player will be controlled by the human playing this game (called "User" from here on) and another by a computer. Each player has 10 points of health and use their Legends to deal damage to their opponent, but each player can generate Barricade segments that stack vertically each for 1 unit of height, blocking 1 point of damage to themselves for each Barricade segment, and subsequently destroying 1 segment of the Barricade for each point of damage if the Barricade is hit. The height of the Barricade dynamically changes as Barricade segments are added or destroyed, with a max possible height of 5 Barricade segments. The Legends have an energy bar split into a number of segments - when the bar filled with generated segments of their respective red and blue energy types, the Legend performs a set action based on their proficiency. To generate energy or Barricade segments, on each player's half of the board they have a set of 5 consecutive individually spinning wheels, like a slot machine, in the center that have panels of symbols encircling each of the wheels so that the wheels display 1 panel of symbol(s) face up at a time which will be considered the current, active symbol(s). Each player's Red and Blue Legends reside on the left and right of their set of wheels, respectively. The symbol(s) displayed on each panel of the slot-machine-like wheels are of 1 of 3 types, (1) 1 or 2 red energy symbols, (2) 1 or 2 blue energy symbols, or (3) 1 or 2 Barricade symbols. Each player's actions resolve on the same turn at the end of the turn, and each turn consists of each player getting 3 separate spins of the set of wheels. The Red or Blue Legend can perform their action if they have a full energy bar of energy segments of their respective color at the end of the turn. 1 energy segment of a certain color is generated if 3 energy symbols of that color are the active symbols on the player that controls them's set of wheels at the end of the turn, with another segment being generated for each additional number of active symbols displayed. This is termed "3oak+" and an example would be that after a player has taken their 3 spins for the turn, and their wheels display 3 currently active red symbols they generate 1 red energy segment for their Red Legend - if their wheels displayed 4 active red symbols they would generate 2 red energy segments, 5 active red symbols would generate 3 red energy segments, and so on. The same "3oak+" scheme applies to any active Barricade symbols on the wheels. If a Legend performs their action, their energy bar is reset to being empty with 0 energy segments, but the number of energy segments persist between turns. On each separate spin of the set of wheels, each player can toggle whether they would like a particular wheel to be locked so that it is not spun on the next of the 3 spins in the turn and the currently active symbols on that wheel remain active - if a wheel is locked on one of those 3 spins it can be unlocked on a later spin if chosen so by the player. Each Legend's a energy bar will maintain the  Each player's sets of wheels are partially covered so that they are only visible to the player on the respective side of the board, and therefore cannot be seen by the player sitting across from them, while the turn is ongoing and the players are spinning their wheels. The game will generally be displayed from the User's perspective, so that they see their set of wheels at all times. At the end of the turn, both player's wheels are fully visible to both players and any Legends on the board take their actions, if they can based on the results of spinning the wheels. When Legends attack, they will generally first be blocked by any number Barricade segments before they can do damage to the opponent's health - although there are exceptions based on the specific actions of each proficiency. For example, when the Knight proficiency (further explained below) performs its action, it attacks once - if the opponent has at least 1 Barricade segment, the entire attack will be taken by the Barricade segment(s) and the opponent will take no damage, but if the opponent has 0 Barricade segments the Knight's attack will deal full damage to the opponent's health.


Template for proficiency action:

-(Proficiency name): (number of energy segments needed to take their action)
---(what this proficency's action does)

The following list describes each proficiency following the above template:

-Knight: 3
---Attacks. Does 3 points of damage to the opponent's health or 3 points of damage to the opponent's Barricade. Will always hit the opponent's barricade first if they have any Barricade segments.

-Wizard: 5
---Attacks 2 separate times. Each attack does 2 points of damage to the opponent's health or 2 points of damage to the opponent's Barricade. The first attack will hit the opponent's Barricade if they have any Barricade segments, and the second attack flies above any Barricade segments so that it always hits the opponent's health regardless of how many Barricade segments the opponent has.

-Bowman: 4
---Attacks. Does 3 points of damage to the opponent's health or 1 point of damage to the opponent's Barricade. Flies at a height of 3 units so that if the opponent's Barricade is at 2 segments or less, it hits the opponent's health anyways.


What does fun mean in this game:


Inspirations:
The Wheels minigame featured in Sea of Stars (game)

Useful links:

> 

## The tool/game
What is your high level vision for the game: 


Target Audience: 


What devices do you want to play this on:

## Done means
Conditions that define when the first minimum viable product is completed and ready for evaluation:

Define targets for the final product:
Define low-targets (features you'd be embarrased not to have)
Define medium targets  (intent of what you are making) 
high targets (polish that means the game is ready to show others)
sretch goals (not sure are possible at the moment  but would be cool)
- 

## Decisions that stay with the person
Define decisions that the coding agent may NOT make without first checking with the human co-designer.

- 

## Accessibility
How do assets carry information in ways besides colour (shape, pattern, brightness, text): 

Input devices tool/game must work with:

## Style

A description of the look of the tool / game:

A tavern-themed game set in the middle ages. UI should be old middle age style parchment, scrolls, wood, and other simple materials. Board and pieces should be fully made of carved wood of various styles to make pieces visually distinct.

Inspirations for the style:

## Words
Specific terms that must appear in the game:


## Autonomy

One paragraph: what the agent may do alone, what it must ask about, and what it may never do. State whether the plan is approved in advance and whether the agent may publish.

The model is pre-approved to ask the user to get github access, and when it does, it creates a public github repo for version control and a public github pages deployment to a website.

## Journal

Design journal will follow the ape-race style, producing mockups, style guides, flow diagrams, maps, animations, comparisons against inspirations, using the text above in this brief.

## Allowed core dependencies

`serde`, `serde_json` (reference default). Add others only here, with the reason.
