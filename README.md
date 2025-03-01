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

## TODOs

- [x] Basic Bevy Template
- [ ] Game Screen (like Start, Pause, Keys menu)
- [ ] Level Design (Parkour, etc.)
- [ ] Player
    - [ ] Time Slow Ability _+ Other Abilities_
    - [ ] Speed Increase
    - [ ] Movement
        - [x] Basic Movement _(like Up, Right and Left)_
        - [ ] Sync `X` Camera Movement
- [ ] Shadow
    - [ ] Mimic Player Moves
    - [ ] Impact from Player Ability
- [ ] Characters & Theme
- [ ] Animations
- [ ] Music
