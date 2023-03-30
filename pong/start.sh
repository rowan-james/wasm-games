#!/usr/bin/env bash

(cd game; wasm-pack build --target web -d ../www/pkg) && python3 -m http.server -d www
