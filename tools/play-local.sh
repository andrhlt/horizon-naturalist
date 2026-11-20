#!/bin/sh
set -eu
project=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
version="${1:-$HOME/Library/Application Support/horizon-launcher/versions/nightly-2026-11-13-9f42064}"
resources="$version/.local/horizon-client/26.3"
game_dir="$project/.local/game"
mkdir -p "$game_dir"
exec "$version/horizon-ctl" --socket "$game_dir/deer.sock" launch \
  --game "$project/dist/horizon-client" --resources "$resources" \
  --home "$game_dir" --game-dir "$game_dir" --dev-mod "$project/.local/content" \
  --world "Naturalist Deer" --wait
