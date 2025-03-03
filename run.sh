#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 [wsl|wasm|lint|auto]"
    echo "auto - Run the game via _cargo run with some debugging features"
    echo "wsl  - Run the game in WSL2 Environment"
    echo "wasm - Run the game in web browser"
    echo "       Make sure you have installed [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner)"
    echo "       via cargo install wasm-server-runner"
    echo "lint - Runs Cargo Clippy and fmt"
    exit 1
fi

platform=$1

case "$platform" in
    "wsl")
        echo "Running Windows build commands in WSL..."
        cargo build --features "debug" --target x86_64-pc-windows-gnu
        cp target/x86_64-pc-windows-gnu/debug/shadow-runner.exe .
        clear
        exec ./shadow-runner.exe "$@"
        ;;
    "wasm")
        echo "Running WASM build commands..."
        clear && cargo run --features debug --target wasm32-unknown-unknown
        ;;
    "lint")
        clear && cargo fmt && cargo clippy --workspace --all-targets --all-features -- -Dwarnings
        ;;
    "auto")
        clear && cargo fmt && cargo clippy --workspace --all-targets --all-features -- -Dwarnings
        ;;
    *)
        echo "Error: Unknown command. Use wsl, wasm, lint or auto"
        echo ""
        echo "`./run.sh`"
        exit 1
        ;;
esac