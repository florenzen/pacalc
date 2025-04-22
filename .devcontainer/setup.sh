#!/bin/bash

npm install

# Trunk expects a tailwindcss executable in the PATH
mkdir -p /home/vscode/.local/bin
cat << 'EOF'  > /home/vscode/.local/bin/tailwindcss
#!/bin/bash
npx @tailwindcss/cli $@
EOF
chmod +x /home/vscode/.local/bin/tailwindcss

# Since some cargo installs are done in the Dockerfile
# some ownerships are root which prevents trunk, e. g.,
# from installing dependencies.
sudo chown -R vscode /usr/local/cargo
