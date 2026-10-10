# bevy_subpixel

A Bevy subpixel smoothing shader for pixel art.

## Why

Commonly used in pixel art games, nearest-neighbor sampling renders
texels at uneven sizes that shift as the camera moves, making pixels
change in size.

This crate replaces Bevy's sprite shader with one that smooths pixels,
keeping their exact size, and camera movement becomes smooth at subpixel
precision.

The technique used in the shader is explained in this video: [Crafting a Better Shader for Pixel Art Upscaling](https://www.youtube.com/watch?v=d6tp43wZqps).

## Usage

Add the dependency to `Cargo.toml`:

```toml
[dependencies]
bevy_subpixel = "0.1"
```

Add `SubpixelSpritePlugin` to the app after `DefaultPlugins`:

```rust
use bevy::prelude::*;
use bevy_subpixel::SubpixelSpritePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SubpixelSpritePlugin))
        .run();
}
```

## Compatibility

| bevy_subpixel | Bevy |
| ------------- | ---- |
| 0.1           | 0.19 |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
