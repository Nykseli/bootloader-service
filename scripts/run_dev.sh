#!/bin/bash

set -e

if [[ ! -d "tmp" ]]; then
    mkdir tmp
    cp /etc/default/grub tmp
fi

cargo run --features dev -- --session
