# Shadow Runner

A WIP Game for [Code for a Cause](https://itch.io/jam/code-for-a-cause)

## Development for WASM

1. Make sure you have installed [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner) tool.
    ```bash
    $ cargo install wasm-server-runner
    ```

2. Run the Game:
    ```bash
    $ cargo run --features debug --target wasm32-unknown-unknown
    ```

## Debug Tools

This game comes with it's own in-built basic editor enabled by `debug` feature
and unlocks the follows keybinds:

|Key Code                                                                           |Description      |
|-----------------------------------------------------------------------------------|-----------------|
|<kbd>Shift</kbd> + <kbd>W</kbd>, <kbd>Option</kbd> + <kbd>W</kbd> or <kbd>8 ↑</kbd>|Move Camera Up   |
|<kbd>Shift</kbd> + <kbd>S</kbd>, <kbd>Option</kbd> + <kbd>S</kbd> or <kbd>2 ↓</kbd>|Move Camera Down |
|<kbd>Shift</kbd> + <kbd>D</kbd>, <kbd>Option</kbd> + <kbd>D</kbd> or <kbd>6 →</kbd>|Move Camera Right|
|<kbd>Shift</kbd> + <kbd>A</kbd>, <kbd>Option</kbd> + <kbd>A</kbd> or <kbd>4 ←</kbd>|Move Camera Left |
|<kbd>Shift</kbd> + <kbd> </kbd>, <kbd>Option</kbd> + <kbd> </kbd> or <kbd>5</kbd>  |Reset Camera     |
|<kbd>+</kbd>                                                                       |Zoom In Camera   |
|<kbd>-</kbd>                                                                       |Zoom Out Camera  |

Additionally, you can also **Left Click** any component, to find it's size and position as well
as you can move it by holding <kbd>Shift</kbd> and draging it by **Left Click**. _Currently Broken_

_I am also thinking of making these debug tools a part of the game and allow people to use them
and create custom map._

## TODOs

- [x] Basic Bevy Template
- [ ] Game Screen (like Start, Pause, Keys menu)
    - [x] Game Pause Mechanism
- [ ] Level Design (Parkour, etc.)
- [ ] Score System
- [ ] Player
    - [ ] Time Slow Ability _+ Other Abilities_
    - [ ] Speed Increase
    - [x] Movement
        - [x] Basic Movement _(like Up, Right and Left)_
        - [x] Sync `X` Camera Movement
- [ ] Shadow
    - [ ] Mimic Player Moves
    - [ ] Impact from Player Ability
- [ ] Characters & Theme
- [ ] Animations
- [ ] Music
