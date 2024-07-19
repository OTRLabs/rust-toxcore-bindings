#!/bin/env bash
rm -rf ./c-toxcore
git clone https://github.com/TokTok/c-toxcore.git
cd c-toxcore
mkdir _build && cd _build
cmake ..
make
sudo make install