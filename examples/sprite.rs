use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_subpixel::SubpixelSpritePlugin;

const SPRITE_SIZE: u32 = 16;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SubpixelSpritePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 288.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn(Sprite::from_image(images.add(checkerboard_image())));
}

fn move_camera(time: Res<Time>, mut camera: Single<&mut Transform, With<Camera2d>>) {
    let t = time.elapsed_secs() * 0.5;
    camera.translation = Vec3::new(t.cos(), t.sin(), 0.0) * 20.0;
}

fn checkerboard_image() -> Image {
    let data = (0..SPRITE_SIZE * SPRITE_SIZE)
        .flat_map(|i| {
            let (x, y) = (i % SPRITE_SIZE, i / SPRITE_SIZE);
            if (x + y) % 2 == 0 {
                [255, 255, 255, 255]
            } else {
                [0, 0, 0, 255]
            }
        })
        .collect();

    Image::new(
        Extent3d {
            width: SPRITE_SIZE,
            height: SPRITE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
