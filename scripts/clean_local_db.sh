#!/bin/bash

set -e
cd $(dirname "$0")
cd ..

rm  tmp/bootkit.db || true
