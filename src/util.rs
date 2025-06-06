use bevy::prelude::*;

#[derive(Component)]
pub(crate) enum ScaledRelativeToWindow {
    BothAxes(f32),
    ByWidth(f32),
}

pub(crate) fn scale_sprites_relative_to_window(
    mut query: Query<(&mut Transform, &Sprite, &ScaledRelativeToWindow)>,
    images: Res<Assets<Image>>,
    window: Single<&Window>
) {
    for (mut transform,sprite, rescale_factor) in query.iter_mut() {
        if let Some(image) = images.get(sprite.image.id()) {
            match rescale_factor {
                ScaledRelativeToWindow::BothAxes(final_factor) => {
                    let change = (window.physical_size().as_vec2()/image.size_f32())*final_factor;
                    transform.scale = change.extend(0.0);
                }
                ScaledRelativeToWindow::ByWidth(final_factor) => {
                    let change = (window.physical_size().x as f32 / image.width() as f32)*final_factor;
                    transform.scale = Vec3::from_array([change,change,0.]);
                }
            }
        }
        
    }   
}