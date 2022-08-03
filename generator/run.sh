#!/bin/sh
python3 gen.py | rustfmt --emit stdout /dev/stdin | tail -n +2 > ../server/src/test.rs