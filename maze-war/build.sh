#!/usr/bin/env bash

(cd game; wasm-pack build --target web -d ../web/build)
