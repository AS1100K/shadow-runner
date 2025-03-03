#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 [wsl|windows|mac|wasm|linux|lint]"
    exit 1
fi

platform=$1

case "$platform" in
    "wsl")
        echo "Running Windows build commands in WSL..."
        cargo build --features "debug,wayland" --target x86_64-pc-windows-gnu
        echo "Build Finished Clearing Terminal and Starting the Game"
        sleep 1 && clear
        exec target/x86_64-pc-windows-gnu/debug/shadow-runner.exe "$@"
        ;;
    "mac")
        echo "Running Mac build commands..."
        # Add your Mac-specific commands here
        ;;
    "wasm")
        echo "Running WASM build commands..."
        clear && cargo run --features debug --target wasm32-unknown-unknown
        ;;
    "linux")
        echo "Running Linux build commands..."

        ;;
    "lint")
        clear && cargo clippy --workspace --all-targets --all-features -- -Dwarnings
        ;;
    *)
        echo "Error: Unknown platform. Use windows, mac, or wasm"
        exit 1
        ;;
esac