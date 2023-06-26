#!/usr/bin/bash

cargo build --release
cp ./target/release/giflar ./
./giflar --inpath ../Bergen_full.sos --outdir ../datfiles/
