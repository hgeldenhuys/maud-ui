#!/usr/bin/env bash
# gallery.sh — start the showcase gallery on a port that is actually free,
# print the URL, and wait until it genuinely serves before returning.
#
# Why this exists: starting the gallery by hand is a three-mistake trap, and a
# 2026-07-27 session hit all three in one sitting.
#
#   1. PORT COLLISION. `examples/showcase.rs` defaults to :3456 and a session
#      had been using :3499. Another project's dev server (kapable-comms) took
#      :3499 the moment the gallery was restarted, then answered every route
#      with an empty 404. That reads exactly like a broken gallery — two audit
#      agents reported the site as down. This script refuses to start on a busy
#      port and picks the next free one instead.
#   2. `setsid` IS NOT ON macOS. The obvious "detach it properly" incantation
#      silently fails, and the server never starts at all.
#   3. READY != LISTENING. Returning as soon as the port binds hands you a
#      window where curl works and a browser gets a connection error, which
#      then caches in the browser as a hard failure page. This polls a real
#      route, not the socket.
#
# Usage:
#   scripts/gallery.sh              # pick a free port from 3456 upward
#   scripts/gallery.sh 3600         # start at this port (still skips if busy)
#   scripts/gallery.sh --stop       # stop any gallery this script started
#   scripts/gallery.sh --status     # what's running, and on what
#
# Exits non-zero with a reason. Never leaves you guessing whether it worked.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

PIDFILE=".gallery.pid"
LOGFILE=".gallery.log"

port_busy() { lsof -nP -iTCP:"$1" -sTCP:LISTEN >/dev/null 2>&1; }

port_owner() {
  lsof -nP -iTCP:"$1" -sTCP:LISTEN 2>/dev/null | awk 'NR==2 {print $1" (pid "$2")"}'
}

case "${1:-}" in
  --stop)
    if [[ -f "$PIDFILE" ]] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
      kill "$(cat "$PIDFILE")" && rm -f "$PIDFILE"
      echo "gallery stopped"
    else
      rm -f "$PIDFILE"
      echo "no gallery running (from this script)"
    fi
    exit 0
    ;;
  --status)
    if [[ -f "$PIDFILE" ]] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
      echo "gallery running: pid $(cat "$PIDFILE")"
      grep -o 'http://[^ ]*' "$LOGFILE" 2>/dev/null | tail -1
    else
      echo "no gallery running (from this script)"
    fi
    exit 0
    ;;
esac

START_PORT="${1:-3456}"

# Find a free port. Announce the skip — a silently different port is its own trap.
PORT=""
for candidate in $(seq "$START_PORT" $((START_PORT + 20))); do
  if port_busy "$candidate"; then
    echo "  port $candidate busy — held by $(port_owner "$candidate")" >&2
  else
    PORT="$candidate"
    break
  fi
done

if [[ -z "$PORT" ]]; then
  echo "error: no free port in ${START_PORT}..$((START_PORT + 20))." >&2
  echo "       Free one, or pass a different start port: scripts/gallery.sh 3600" >&2
  exit 1
fi

# Build first, in the foreground, so a compile error is a compile error rather
# than a mysteriously absent server.
echo "building showcase…" >&2
if ! cargo build --example showcase --quiet 2>&1 | tail -20; then
  echo "error: cargo build failed — fix that before starting the gallery." >&2
  exit 1
fi

ADDR="127.0.0.1:$PORT" nohup cargo run --example showcase >"$LOGFILE" 2>&1 &
echo $! > "$PIDFILE"

# Poll a REAL route. `/` proves routing and templates work, not just the socket.
for _ in $(seq 1 60); do
  if curl -fs -o /dev/null --max-time 2 "http://localhost:$PORT/"; then
    echo
    echo "  gallery ready → http://localhost:$PORT/"
    echo "  stop with     → scripts/gallery.sh --stop"
    echo
    echo "  NOTE: use localhost, not 127.0.0.1 — a browser that hit 127.0.0.1"
    echo "        while the port was down caches the failure page and keeps"
    echo "        showing it after the server is back."
    exit 0
  fi
  sleep 1
done

echo "error: server did not answer on http://localhost:$PORT/ within 60s." >&2
echo "       Last log lines:" >&2
tail -15 "$LOGFILE" >&2
exit 1
