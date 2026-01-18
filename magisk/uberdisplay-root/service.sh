#!/system/bin/sh
# Late-start service hook for the UberDisplay root module.

MODDIR="${0%/*}"
BIN_DIR="$MODDIR/common/bin"
SOCKET_DIR="/data/local/tmp/uberdisplay"
LOG_FILE="/data/local/tmp/uberdisplay/root-module.log"

mkdir -p "$SOCKET_DIR"

if [ -x "$BIN_DIR/uberdisplay-rootd" ]; then
  "$BIN_DIR/uberdisplay-rootd" --socket "$SOCKET_DIR/root.sock" >> "$LOG_FILE" 2>&1 &
else
  echo "$(date) - uberdisplay root service placeholder (no daemon)" >> "$LOG_FILE"
fi
