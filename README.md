# Bevy Pong

Reimplementing pong in Bevy 0.14 to learn how this nonsense works

Started out following [this tutorial](https://taintedcoders.com/bevy/pong-tutorial/#moving-our-ball) to get started, but is gradually branching out from there to develop a better understanding of the engine.

---

## Changes made from tutorial

- Minor structuring and ordering changes in systems and the like
- Format of scoreboard changed to display a single centered scoreboard with the format `{ai_score} - {player_score}`
- Restructured project to be based on rust modules and Bevy's plugin system for extensibility and developability. Modules and assocaited plugins are:
    - `bevy-pong::structure` - Components, Events, Enums, etc, used by the rest of the program
    - `bevy-pong::setup`/`setup::PongInitPlugin` - Initialization systems to create and setup needed Entities and the like
    - `bevy-pong::actors`/`actors::PongActorsPlugin` - Manages 'active' entities and behavior, such as AI, input and movement handling, and the ball
    - `bevy-pong::score`/`score::PongScorePlugin` - Handles ball loss, score updates, the scoreboard, ball resets, and everything else related to scoring
    - `bevy-pong::gamestate`/`gamestate::PongGameStatePlugin` - Handles core internal functionality like collsions, and converting between custom component and engine-native ones.
- AI movement uses an easing function to provide smoother behavior, and to make them beatable
- The collision system now ejects the ball to the closest point outside the bounding box to prevent mistaken collisions
- The window has basic settings and configuration attached i.e. window title, set size, disable maximizing, etc.

## Planned next steps

- [x] Make the AI less jittery
- [X] Clean up collision to actually eject the ball instead of just reversing direction to avoid edge cases
- [x] Vary hit angle based on how close to the center of the paddle you were (Needs continuous collision to prevent clipping, pending migration to avian physics)
- [ ] Win state
- [x] Make the AI beatable
- [ ] Use `bevy::ui` instead of using hacked together FontBundle2d UI
- [ ] Work out bevy scenes
- [ ] Make a menu
- [x] Add some sprites to things
- [x] Make the window a little more consistent
- [x] Migrate to an actual collision handler
- [ ] Add some shaders and polish

---

## Licensing

Copyright 2024 Mia A.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
