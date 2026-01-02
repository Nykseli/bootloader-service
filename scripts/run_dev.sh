#!/bin/bash

set -e
cd $(dirname "$0")
cd ..

if [[ ! -e "tmp/grub" ]]; then
    mkdir -p tmp
    cp test_data/grub.cfg tmp
    cp test_data/grub_full tmp/grub
    cp test_data/grubenv_empty tmp/grubenv

    scripts/setup_local_db.sh
fi

if [[ -z $DATABASE_URL ]]; then
    echo env DATABASE_URL is not set
    echo set it and run this again
    echo "run ./scripts/setup_local_db.sh if you're not sure how to set it"
    exit 1
fi

cargo run --features dev -- --session --pretty
