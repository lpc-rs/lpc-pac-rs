#!/bin/bash

# Bash script to (re)generate Platform Access Crates (PACs) with svd2rust.
# For PAC to be generated, the following items are required:
#
# - It needs to be registered below in the `PACS` array.
# - The directory needs to contain an SVD file with the name matching
#   the crate name.

set -euo pipefail


### Configuration

PACS=(
    lpc11uxx
    lpc176x5x
)


### Helper functions

RED="\e[31m"
CYAN="\e[36m"
BOLDGREEN="\e[1;32m"
RESET="\e[0m"

cecho() {
    # Colorized echo
    echo -e "${1}${2}${RESET}"
}

fail() {
    cecho $RED "Error: $2"
    exit $1
}

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

require_command() {
    if ! command_exists $1; then
        fail 1 "Command $1 not found."
    fi
}


### Main

require_command svd2rust
require_command cargo-fmt

generate_pac() {
    cecho $CYAN "Running svd2rust..."
    svd2rust -i "${1}.svd" 2> >(tee svd2rust-warnings.log >&2)
    mv lib.rs src/lib.rs
    cecho $CYAN "Formatting generated code..."
    cargo fmt
}

echo "Using $(svd2rust --version)"
for PAC in ${PACS[*]}; do
    cecho $BOLDGREEN "\nEntering $PAC/"
    pushd "$PAC" >/dev/null
    generate_pac "$PAC"
    cecho $BOLDGREEN "Done"
    popd >/dev/null
done
echo ""
