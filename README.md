# Rust Synth 

A basic VST Synth written in Rust including editor made with `egui`.
This is a wavetable synth. The wavetable can be swept via an envelope.

The `wave warp` parameter indexes the wavetable.
The `warp ratio` parameter controls how much the warp envelope modifies the wave warp.
By default the `warp ratio` is set to 0.5, meaning that the warp envelope has no effect.
When the `warp ratio` is > 0.5, the table will be swept upward depending on the warp envelope.

In general, this plugin has a lot of problems. There are a couple of strange artifacts
and the filter does not work properly.
The structure of the code can be greatly improved and will be in the upcoming SynthTwo.

# Usage

Build with `cargo build`. This will produce a binary in `synthone/target/debug/libSynthOne.so`.
Find some VST host (like `Carla`) and point the host to that plugin. One way to do this is to
copy the `SynthOne` binary into `~/.vst` (assuming you are on Linux). You can also use just
about any DAW as your plugin host. 
