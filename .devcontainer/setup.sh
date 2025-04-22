#!/bin/bash

cargo install trunk
cargo install leptosfmt
rustup target add wasm32-unknown-unknown
npm install

# Trunk expects a tailwindcss executable in the PATH
cat << EOF  > /home/vscode/.local/bin/tailwindcss
#!/bin/bash
npx @tailwindcss/cli $@
EOF
chmod +x /home/vscode/.local/bin/tailwindcss
