#!/bin/bash
# CyberLab Linux Setup Script

set -e

echo "--- CyberLab Linux Setup ---"

# 1. Check for Docker
if ! command -v docker &> /dev/null; then
    echo "[!] Docker not found. Please install Docker: https://docs.docker.com/engine/install/"
    exit 1
fi

# 2. Check for Rust (Optional if running pre-built, but good for dev)
if ! command -v cargo &> /dev/null; then
    echo "[!] Rust/Cargo not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# 3. Ensure PATH is setup
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/env" ]; then
        echo "[*] Sourcing cargo environment..."
        source "$HOME/.cargo/env"
    else
        echo "[!] Cargo not found. Is it installed?"
    fi
fi

# Add to .zshrc if using zsh and not already there
if [[ "$SHELL" == *"zsh"* ]]; then
    if ! grep -q ".cargo/bin" "$HOME/.zshrc"; then
        echo "[*] Adding cargo to .zshrc..."
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.zshrc"
    fi
elif [[ "$SHELL" == *"bash"* ]]; then
    if ! grep -q ".cargo/bin" "$HOME/.bashrc"; then
        echo "[*] Adding cargo to .bashrc..."
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
    fi
fi

# 4. Setup Docker permissions
echo "[*] Adding current user to 'docker' group (may require sudo)..."
sudo usermod -aG docker $USER || true

# 4. Install dependencies for backend (Ubuntu/Debian example)
if command -v apt-get &> /dev/null; then
    echo "[*] Installing build dependencies..."
    sudo apt-get update && sudo apt-get install -y build-essential pkg-config libssl-dev
fi

# 5. Build Backend
echo "[*] Building Backend..."
cd backend && cargo build --release
cd ..

# 6. Verify Security
echo "[*] Running security verification..."
./tests/verify_sandbox.sh

echo "--------------------------------"
echo "[+] Setup complete!"
echo "[+] To run the game:"
echo "    1. Start the backend: ./backend/target/release/cyberlab-backend"
echo "    2. Launch the Unreal Engine client."
echo "--------------------------------"
