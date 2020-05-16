#!/usr/bin/env bash

set -eu

cargo build --release
./target/release/chambray -w 1400 -h 700
eog image.ppm

