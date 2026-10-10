use std::path::{Path, PathBuf};

use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::prelude::*;

const BEVY_SPRITE_SHADER_PATH: &str = "bevy_sprite_render/render/sprite.wgsl";

pub struct SubpixelSpritePlugin;

impl Plugin for SubpixelSpritePlugin {
    fn build(&self, app: &mut App) {
        app.world()
            .resource::<EmbeddedAssetRegistry>()
            .insert_asset(
                PathBuf::new(),
                Path::new(BEVY_SPRITE_SHADER_PATH),
                include_bytes!("../assets/subpixel_sprite.wgsl"),
            );
    }
}
