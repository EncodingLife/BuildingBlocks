use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

const SIZE: (u32, u32) = (2560, 2560);

#[derive(Resource)]
pub struct MapImage {
    texture: Handle<Image>,
}

pub(super) fn setup_map(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let e = Extent3d {
        width: SIZE.0,
        height: SIZE.1,
        depth_or_array_layers: 1,
    };
    let image = Image::new_fill(
        e,
        TextureDimension::D2,
        &[255, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );

    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.insert_resource(MapImage { texture: image });
}
