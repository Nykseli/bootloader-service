#!/bin/bash

set -e
cd $(dirname "$0")
cd ..

if [[ ! -d "tmp" ]]; then
    mkdir tmp
    cp /etc/default/grub tmp

    touch tmp/bootloader.db
    sqlite3 tmp/bootloader.db < db/grub2.sql
fi

cargo run --features dev -- --session
