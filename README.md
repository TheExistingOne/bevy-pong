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
- [ ] Make gameplay a little more interesting (vary the ejection angle based on how close to the center of the paddle you were)
- [ ] Win state
- [x] Make the AI beatable
- [ ] Use `bevy::ui` instead of using hacked together FontBundle2d UI
- [ ] Work out bevy scenes
- [ ] Make a menu
- [ ] Add some sprites to things
- [x] Make the window a little more consistent
- [ ] Migrate to an actual collision handler
- [ ] Add some shaders and polish

---

## Licensing

Code is unlicensed since this is a private repo, and I'd technically have to reach out to the original tutorial author to get permission to license since they don't specify one which means All Rights Reserved by default.
