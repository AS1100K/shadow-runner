# Contributing

Hi there, this files not only guides you as an Contributor but also how this project technicality.

## Getting Started

Make sure you have installed rust and cloned the repo. There you will find `run.sh` file, this is a useful
shell script with the following commands:

- `./run.sh wsl` - Runs the game with debug feature, if you are on WSL
- `./run.sh wasm` - Runs the game in the web with debug features
- `./run.sh auto` - Runs the game on current selected target with debug features
- `./run.sh lint` - Runs `cargo fmt` and `cargo clippy`

## Project Structure

- `.github/` - CI/CD Files
- `assets/` - Game assets, they are passed as a whole with every game release
- `src/` - Code
- `wasm/` - Some helper files to run your wasm in browser
- `run.sh` - utility script

_// TODO_
