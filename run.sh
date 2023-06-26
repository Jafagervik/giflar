#!/usr/bin/bash

cargo build --release
cp ./target/release/giflar ./
./giflar --filename ../Bergen_full.sos --outdir ../datfiles/
