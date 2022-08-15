#!/bin/sh
python3 gen.py | rustfmt --config max_width=200 --emit stdout /dev/stdin | tail -n +2 > ../server/src/test.rs
