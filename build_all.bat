rem Build libs
cargo build -p chui_core
cargo build -p chui_error
cargo build -p chui_macros
cargo build -p chui_ui

rem Build bins
cargo build -p chui-console
cargo build -p chui-trainer
cargo build -p chui-ui
