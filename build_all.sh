#!/bin/bash

# Build libs
cargo build -p chui_core
cargo build -p chui_ui
cargo build -p chui_macros

# Build bins
cargo build -p chui-console
cargo build -p chui-ui
cargo build -p chui-trainer
