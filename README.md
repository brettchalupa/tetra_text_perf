# Tetra Text Perf Text

Testing out drawing text on the screen with [Tetra 0.8.0](https://crates.io/crates/tetra).

[View the opened GitHub Issue.](https://github.com/17cupsofcoffee/tetra/issues/347)

Clone and run with: `cargo run` or `cargo run --release`

## Results

I'm on an Apple M2 Pro running on a MacBook Pro.

debug build results:

- 16 texts runs at a steady 120 FPS
- 17 texts causes frames to start dropping a little bit, ~117 FPS
- 40 texts halves the FPS to about 60 FPS
- 50 texts drops to about 40 FPS

Happens with both the Vector (ttf) and Bitmap font from the example: https://github.com/17cupsofcoffee/tetra/blob/1d23de2c919f8b9106c9d703076935464079c8aa/examples/text.rs

Noticing similar behavior with `cargo run --release` as well.

Drawing the same `Text` instance multiple times does not impact performance.

## Why this isn't good

It's not uncommon to render a lot of text in a game. For example, while building out a dev tool that renders some coordinates, I noticed the frames start to drop.
