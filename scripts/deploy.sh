#!/usr/bin/env bash
# -----------------------------------------------------------------------------
# 5BX Web Application Smart Deployment Script
# -----------------------------------------------------------------------------
# Supports two deployment modes to strictly minimise mobile cellular data:
#   1. Remote Build (Default): Builds frontend dist locally (~0.9s), streams
#      source + dist (<70 KB). Penguinplex compiles Rust natively using home
#      broadband and 30 GB RAM.
#   2. Local Build (--local): Compiles release binary on laptop, compresses,
#      and pushes standalone binary (~8 MB).
# -----------------------------------------------------------------------------
set -euo pipefail

REMOTE_HOST="${REMOTE_HOST:-penguinplex}"
REMOTE_USER="${REMOTE_USER:-matthew}"
REMOTE_DIR="${REMOTE_DIR:-/home/${REMOTE_USER}/5bx-web}"
SERVICE_PORT=8085
MODE="remote"

if [[ "${1:-}" == "--local" ]]; then
  MODE="local"
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "================================================================="
echo "   RCAF 5BX WEB APPLICATION DEPLOYMENT"
echo "================================================================="
echo "Target Host : ${REMOTE_HOST}"
echo "Target User : ${REMOTE_USER}"
echo "Target Path : ${REMOTE_DIR}"
if [[ "$MODE" == "remote" ]]; then
  echo "Deploy Mode : Remote Build (Lowest Cell Data: ~60 KB)"
else
  echo "Deploy Mode : Local Binary Push (~8 MB)"
fi
echo "-----------------------------------------------------------------"

if [[ "$MODE" == "remote" ]]; then
  echo "🚀 Building frontend locally (~1s)..."
  cd "$PROJECT_ROOT/frontend"
  npm run build

  echo "📦 Preparing source stream (<70 KB)..."
  cd "$PROJECT_ROOT"

  tar --exclude="backend/target" \
      --exclude="frontend/node_modules" \
      --exclude=".git" \
      --exclude="*.db" \
      -czf - \
      backend/src backend/Cargo.toml backend/migrations \
      frontend/dist \
      scripts \
      .env.example .gitignore README.md | ssh "${REMOTE_USER}@${REMOTE_HOST}" '
    set -euo pipefail
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "📥 Unpacking source files on server..."
    mkdir -p "'"${REMOTE_DIR}"'"
    cd "'"${REMOTE_DIR}"'"
    tar -xzf -

    if [ ! -f .env ]; then
      if [ -f .env.example ]; then
        cp .env.example .env
        echo "⚠️ Created initial template .env from .env.example on remote server."
        echo "Please edit ${REMOTE_DIR}/.env to configure your DATABASE_URL and JWT_SECRET."
      else
        echo "❌ FATAL: No .env file found at ${REMOTE_DIR}/.env"
        exit 1
      fi
    fi

    echo "🦀 Compiling Rust binary natively on server..."
    cd backend
    cargo build --release
    install -m 755 target/release/fivebx-server ../fivebx-server
    cd ..

    echo "🔄 Restarting fivebx systemd user service..."
    mkdir -p ~/.config/systemd/user
    if [ ! -f ~/.config/systemd/user/fivebx.service ]; then
      cat << 'SERVICEDEF' > ~/.config/systemd/user/fivebx.service
[Unit]
Description=Royal Canadian Air Force 5BX Web Application
After=network.target

[Service]
Type=simple
WorkingDirectory=/home/matthew/5bx-web
ExecStart=/home/matthew/5bx-web/fivebx-server
Restart=always
RestartSec=3
EnvironmentFile=/home/matthew/5bx-web/.env

[Install]
WantedBy=default.target
SERVICEDEF
      systemctl --user daemon-reload
      systemctl --user enable fivebx
    fi

    systemctl --user daemon-reload
    systemctl --user restart fivebx
    sleep 1
    if systemctl --user is-active --quiet fivebx; then
      echo "✅ 5BX Server process started successfully under systemd!"
    else
      systemctl --user status fivebx --no-pager
      exit 1
    fi
  '

else
  echo "🚀 Building frontend locally..."
  cd "$PROJECT_ROOT/frontend"
  npm run build

  echo "🦀 Building release binary locally..."
  cd "$PROJECT_ROOT/backend"
  cargo build --release
  strip target/release/fivebx-server

  echo "📦 Compressing binary with gzip..."
  gzip -c9 target/release/fivebx-server > /tmp/fivebx-server.gz
  BIN_SIZE=$(ls -lh /tmp/fivebx-server.gz | awk '{print $5}')
  echo "Compressed size to transfer: ${BIN_SIZE}"

  echo "📤 Pushing compressed binary to ${REMOTE_HOST}..."
  cat /tmp/fivebx-server.gz | ssh "${REMOTE_USER}@${REMOTE_HOST}" '
    set -euo pipefail
    mkdir -p "'"${REMOTE_DIR}"'"
    cd "'"${REMOTE_DIR}"'"
    gunzip -c > fivebx-server
    chmod +x fivebx-server

    if [ ! -f .env ]; then
      if [ -f .env.example ]; then
        cp .env.example .env
        echo "⚠️ Created initial template .env from .env.example on remote server."
      else
        echo "❌ FATAL: No .env file found at ${REMOTE_DIR}/.env"
        exit 1
      fi
    fi

    echo "🔄 Restarting fivebx systemd user service..."
    mkdir -p ~/.config/systemd/user
    if [ ! -f ~/.config/systemd/user/fivebx.service ]; then
      cat << 'SERVICEDEF' > ~/.config/systemd/user/fivebx.service
[Unit]
Description=Royal Canadian Air Force 5BX Web Application
After=network.target

[Service]
Type=simple
WorkingDirectory=/home/matthew/5bx-web
ExecStart=/home/matthew/5bx-web/fivebx-server
Restart=always
RestartSec=3
EnvironmentFile=/home/matthew/5bx-web/.env

[Install]
WantedBy=default.target
SERVICEDEF
      systemctl --user daemon-reload
      systemctl --user enable fivebx
    fi

    systemctl --user daemon-reload
    systemctl --user restart fivebx
    sleep 1
    if systemctl --user is-active --quiet fivebx; then
      echo "✅ 5BX Server process started successfully under systemd!"
    else
      systemctl --user status fivebx --no-pager
      exit 1
    fi
  '
  rm -f /tmp/fivebx-server.gz
fi

echo "🔍 Verifying application health over Tailscale..."
HEALTH_URL="http://${REMOTE_HOST}:${SERVICE_PORT}/api/health"
for i in {1..10}; do
  if curl -s -f "${HEALTH_URL}" > /dev/null 2>&1; then
    echo "================================================================="
    echo "🎉 5BX APPLICATION IS LIVE & HEALTHY ON PENGUINPLEX!"
    echo "   Web App URL : http://${REMOTE_HOST}:${SERVICE_PORT}"
    echo "   Tailscale   : http://100.94.1.36:${SERVICE_PORT}"
    echo "================================================================="
    exit 0
  fi
  sleep 1
done

echo "⚠️  Application deployed; awaiting health check response."
