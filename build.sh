#!/bin/bash
cd home
npm i
npm build
cd ../server
cargo build --target-dir ../server_target
cd ..
