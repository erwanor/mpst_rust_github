#!/bin/sh

set -e

# find ./examples/. -type f -exec sh -c 'for example in "$@"; do (cargo run --example ${example:13:-3}) done' argv0 {} +

# cargo run --example actyx_os_1
# cargo run --example actyx_os_2