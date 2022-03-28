# Rust Synth 

First try at making a VST Synth Plugin with Rust. Loosely inspired by ampli-fe.

# Usage

Build with `cargo build`. This will produce a binary in `synthone/target/debug/libSynthOne.so`.
Find some VST host (like `Carla`) and point the host to that plugin. One way to do this is to
copy the `SynthOne` binary into `~/.vst` (assuming you are on Linux). You can also use just
about any DAW as your plugin host. 
