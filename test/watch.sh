#!/bin/bash

self_dir=$(dirname -- "$0")

cargo watch --clear --watch "${self_dir}/server" --exec "run -- --config-file \"${self_dir}/server/config.toml\"" &

cargo_pid=$!

handle_signal() {
    kill -TERM "$cargo_pid" 2>/dev/null
}

trap 'handle_signal' SIGHUP SIGINT SIGTERM

wait "$cargo_pid"
